use evdev_rs::enums::{EventCode, EventType, EV_ABS, EV_KEY, EV_MSC};
use evdev_rs::{Device, ReadFlag};
use std::fs::File;
use std::path::Path;
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

    loop {
        let ev = d.next_event(ReadFlag::NORMAL).map(|val| val.1);

        match ev {
            Ok(ev) => match (check_holding(), check_resizing()) {
                (true, false) => {
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER) && ev.value == 1
                    {
                        let mut com = Command::new("ydotool");
                        let com = com.args(["key", "0x110:1"]);
                        com.output().unwrap();
                    } else if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER)
                        && ev.value == 0
                    {
                        let mut com = Command::new("ydotool");
                        let com = com.args(["key", "0x110:0"]);
                        com.output().unwrap();
                    }
                }
                (false, true) => {
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER) && ev.value == 1
                    {
                        let mut com = Command::new("ydotool");
                        let com = com.args(["key", "0x111:1"]);
                        com.output().unwrap();
                    } else if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TOOL_FINGER)
                        && ev.value == 0
                    {
                        let mut com = Command::new("ydotool");
                        let com = com.args(["key", "0x111:0"]);
                        com.output().unwrap();
                    }
                }
                (false, false) => {
                    let mut com = Command::new("ydotool");
                    let com = com.args(["key", "0x110:0"]);
                    com.output().unwrap();
                    continue;
                }
                (true, true) => {
                    eprintln!("PLEASE DO NOT ENTER HERE")
                }
            },
            Err(e) => println!("error: {:?}", e),
        }
    }
}

fn check_holding() -> bool {
    Path::new("/home/zbykovd/.config/fusuma/holding").exists()
}

fn check_resizing() -> bool {
    Path::new("/home/zbykovd/.config/fusuma/resizing").exists()
}
