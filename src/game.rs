use db::User;
use std::process::Command;
use std::env;

pub fn play_game(user: &User) {
    // Check for currently running sessions to re-attach
    let reattach = Command::new("screen")
        .arg("-dr")
        .status()
        .unwrap();
    // If there were none, start the game again
    if !reattach.success() {
        Command::new("screen")
            .arg("-S")
            .arg(&user.name)
            .arg("nethack")
            .arg("-d")
            .arg(format!("{}/{}", env::var("PWD").unwrap(), user.name))
            .arg("-u")
            .arg(&user.name)
            .status()
            .unwrap();
    }
}

pub fn generate_skeleton(user: &User) {
    Command::new("/bin/cp")
        .arg("-r")
        .arg("./skeleton")
        .arg(user.name.clone())
        .output()
        .unwrap();
}
