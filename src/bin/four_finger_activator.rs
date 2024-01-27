use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).unwrap();
    match arg.as_str() {
        "holding" => {
            //
            match Path::new("/home/zbykovd/.config/fusuma/holding").exists() {
                true => {
                    std::fs::remove_file("/home/zbykovd/.config/fusuma/holding").ok();
                    let mut com = Command::new("ydotool");
                    let com = com.args(["key", "125:0"]);
                    com.output().unwrap();
                }
                false => {
                    File::create("/home/zbykovd/.config/fusuma/holding").unwrap();
                    let mut com = Command::new("ydotool");
                    let com = com.args(["key", "125:1"]);
                    com.output().unwrap();
                }
            }
        }
        "resizing" => {
            //
            match Path::new("/home/zbykovd/.config/fusuma/resizing").exists() {
                true => {
                    std::fs::remove_file("/home/zbykovd/.config/fusuma/resizing").ok();
                    let mut com = Command::new("ydotool");
                    let com = com.args(["key", "125:0"]);
                    com.output().unwrap();
                }
                false => {
                    File::create("/home/zbykovd/.config/fusuma/resizing").unwrap();
                    let mut com = Command::new("ydotool");
                    let com = com.args(["key", "125:1"]);
                    com.output().unwrap();
                }
            }
        }
        _ => {}
    }
}
