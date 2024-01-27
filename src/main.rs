use evdev_rs::enums::{EventCode, EventType, EV_ABS, EV_KEY, EV_MSC};
use evdev_rs::{Device, ReadFlag};
use std::fs::File;
use std::process::{exit, Command};
use std::time::Instant;

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

    let mut one_finger_inst: Option<Instant> = None;
    let mut one_finger_bool: bool = false;
    let mut one_finger_posx: Option<i32> = None;
    let mut one_finger_posy: Option<i32> = None;
    let mut two_finger_inst: Option<Instant> = None;
    let mut two_finger_bool: bool = false;
    loop {
        let ev = d.next_event(ReadFlag::NORMAL).map(|val| val.1);
        match ev {
            Ok(ev) => {
                match (&ev.event_code, &ev.value) {
                    (EventCode::EV_KEY(EV_KEY::BTN_TOOL_DOUBLETAP), 1) => {
                        if !one_finger_bool {
                            two_finger_inst = Some(Instant::now());
                        }
                        println!("{:?}\t1", ev.event_code)
                    }
                    (EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER), 1) => {
                        one_finger_inst = Some(Instant::now());
                        println!("{:?}\t1", ev.event_code)
                    }
                    (EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER), 0) => {
                        two_finger_inst = None;
                        one_finger_inst = None;
                        one_finger_posx = None;
                        one_finger_posy = None;
                        one_finger_bool = false;
                        let mut com = Command::new("ydotool");
                        let com = com.args(["key", "0x110:0", "0x111:0", "125:0"]);
                        com.output().unwrap();
                        println!("{:?}\t0", ev.event_code)
                    }
                    (&EventCode::EV_ABS(ref abs), &pos) => {
                        // println!("{:?}\t{:?}", ev.event_code, change)
                        match abs {
                            EV_ABS::ABS_X => match one_finger_posx {
                                Some(init_pos) => {
                                    if ((pos - init_pos).abs() > 15) && !one_finger_bool {
                                        one_finger_inst = None;
                                        one_finger_posx = None;
                                        one_finger_posy = None;
                                        two_finger_inst = None;
                                        println!("canceled");
                                    } else if pos < 150 || pos > 1400 - 150 {
                                        one_finger_inst = None;
                                        one_finger_posx = None;
                                        one_finger_posy = None;
                                        two_finger_inst = None;
                                        println!("canceled");
                                    }
                                }
                                None => one_finger_posx = Some(pos),
                            },
                            EV_ABS::ABS_Y => match one_finger_posy {
                                Some(init_pos) => {
                                    if ((pos - init_pos).abs() > 15) && !one_finger_bool {
                                        one_finger_inst = None;
                                        one_finger_posx = None;
                                        one_finger_posy = None;
                                        two_finger_inst = None;
                                        println!("canceled");
                                    } else if pos < 150 || pos > 900 - 150 {
                                        one_finger_inst = None;
                                        one_finger_posx = None;
                                        one_finger_posy = None;
                                        two_finger_inst = None;
                                        println!("canceled");
                                    }
                                }
                                None => one_finger_posy = Some(pos),
                            },
                            _ => {}
                        }
                    }
                    _ => {
                        if let Some(inst) = &one_finger_inst {
                            if (inst.elapsed() > std::time::Duration::from_secs_f64(0.25))
                                & !one_finger_bool
                            {
                                if two_finger_inst.is_none() {
                                    println!("executing one finger");
                                    let mut com = Command::new("ydotool");
                                    let com = com.args(["key", "125:1", "0x110:1"]);
                                    com.output().unwrap();
                                    one_finger_bool = true;
                                } else {
                                    println!("executing two finger");
                                    let mut com = Command::new("ydotool");
                                    let com = com.args(["key", "125:1", "0x111:1"]);
                                    com.output().unwrap();
                                    one_finger_bool = true;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => println!("error: {:?}", e),
        }
    }
}
