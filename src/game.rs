use db::User;
use std::env;
use std::process::Command;

pub fn play_game(user: &User) {
    // Check for currently running sessions to re-attach
    let reattach = Command::new("screen")
        .arg("-d")
        .arg("-r")
        .arg(&user.name)
        .status()
        .unwrap();
    // If there were none, start the game again
    if !reattach.success() {
        Command::new("screen")
            .arg("-S")
            .arg(&user.name)
            .arg("nethack")
            .arg("-d")
            .arg(format!("{}/workspace", env::var("PWD").unwrap()))
            .arg("-u")
            .arg(&user.name)
            .status()
            .unwrap();
    }
}

pub fn play_hunt(user: &User) {
    /*Command::new("ulimit")
    .arg("-Ht")
    .arg("5")
    .arg("hunt")
    .arg("-n")
    .arg(&user.name)
    .arg("scorpionland.net")
    .status()
    .unwrap();*/
    Command::new("bash")
        .arg("-c")
        .arg(format!(
            "ulimit -t10; nice hunt -n {} scorpionland.net",
            &user.name
        ))
        .status()
        .unwrap();
}

pub fn generate_skeleton(user: &User) {
    Command::new("/bin/cp")
        .arg("-r")
        .arg("./skeleton")
        .arg(user.name.clone())
        .output()
        .unwrap();
}
