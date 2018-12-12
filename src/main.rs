extern crate cursive;
extern crate sha2;
extern crate base64;
extern crate rusqlite;
extern crate regex;

use cursive::Cursive;

mod screens;
mod db;

fn main() {
    let mut siv = Cursive::ncurses();

    screens::main_screen(&mut siv);

    siv.run();
}
