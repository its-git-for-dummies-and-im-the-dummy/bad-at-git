use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("ChangeLog.md")
        .expect("Failed to open changelog");
    let ch: u8 = rand::random();
    f.write(&[ch]).expect("Failed to write changelog");
    Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to add");
    Command::new("git")
        .arg("commit")
        .arg("--allow-empty")
        .arg("--allow-empty-message")
        .arg("-m")
        .arg("")
        .output()
        .expect("Failed to commit");
    Command::new("git")
        .arg("push")
        .arg("--force")
        .output()
        .expect("Failed to push");
    println!("Yay, I got good at git!");
}
