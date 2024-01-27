use evdev_rs::enums::{EventCode, EventType, EV_ABS, EV_KEY, EV_MSC};
use evdev_rs::{Device, ReadFlag};
use std::fs::File;
use std::process::{exit, Command};
use std::time::{Duration, Instant};

fn main() {
    // open touchpad device
    let s = "touchpad";
    let mut d = None;
    for event_num in 0..100 {
        let test_d =
            Device::new_from_fd(File::open(format!("/dev/input/event{}", event_num)).unwrap())
                .unwrap();
        if test_d.name().unwrap().to_lowercase().contains(s) {
            d = Some(test_d);
            break;
        }
    }

    if d.is_none() {
        exit(1);
    }
    println!("pass");
    let d = d.unwrap();
    let mut state = State::WaitingActivation;

    loop {
        let ev = d.next_event(ReadFlag::NORMAL).map(|val| val.1);

        match ev {
            Ok(ev) => match state {
                State::WaitingActivation => {
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_TRIPLETAP)
                        && ev.value == 1
                    {
                        state = State::WaitingTryingActivation(Instant::now(), FingerCount::Three);
                        println!("{state:?}");
                    }
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_QUADTAP) && ev.value == 1
                    {
                        state = State::WaitingTryingActivation(Instant::now(), FingerCount::Four);
                        println!("{state:?}");
                    }
                }
                State::WaitingTryingActivation(inst, fc) => {
                    if inst.elapsed() > Duration::from_millis(50) {
                        state = State::WaitingActivation;
                        println!("{state:?}");
                        continue;
                    }
                    if (ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_TRIPLETAP)
                        || ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_QUADTAP))
                        && ev.value == 0
                    {
                        state = State::WaitingAction(Instant::now(), fc);
                        println!("{state:?}");
                    }
                }
                State::WaitingAction(inst, fc) => {
                    if inst.elapsed() < Duration::from_millis(20) {
                        continue;
                    }
                    if inst.elapsed() > Duration::from_millis(1000) {
                        state = State::WaitingActivation;
                        println!("{state:?}");
                        continue;
                    }
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER) && ev.value == 1
                    {
                        match fc {
                            FingerCount::Three => {
                                println!("executing one finger");
                                let mut com = Command::new("ydotool");
                                let com = com.args(["key", "125:1", "0x110:1"]);
                                com.output().unwrap();
                                state = State::Active;
                                println!("{state:?}");
                            }
                            FingerCount::Four => {
                                // TODO FILTER MISCLICKS
                                // TODO FOUR FINGER WORKS PURLY
                                println!("executing two finger");
                                let mut com = Command::new("ydotool");
                                let com = com.args(["key", "125:1", "0x111:1"]);
                                com.output().unwrap();
                                state = State::Active;
                                println!("{state:?}");
                            }
                        }
                    }
                }
                State::Active => {
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER) && ev.value == 0
                    {
                        let mut com = Command::new("ydotool");
                        let com = com.args(["key", "0x110:0", "0x111:0", "125:0"]);
                        com.output().unwrap();
                        state = State::WaitingActivation;
                        println!("{state:?}");
                    }
                }
            },
            Err(e) => println!("error: {:?}", e),
        }
    }
}

#[derive(Debug)]
enum State {
    WaitingActivation,
    WaitingTryingActivation(Instant, FingerCount),
    WaitingAction(Instant, FingerCount),
    Active,
}

#[derive(Debug, Clone, Copy)]
enum FingerCount {
    Three,
    Four,
}
