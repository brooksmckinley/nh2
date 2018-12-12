extern crate cursive;
extern crate sha2;
extern crate base64;
extern crate rusqlite;
extern crate regex;
extern crate rand;

use cursive::Cursive;

pub mod screens;
pub mod db;

fn main() {
    let mut siv = Cursive::ncurses();

    screens::main_screen(&mut siv);

    siv.run();
}
