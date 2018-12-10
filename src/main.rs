extern crate cursive;
extern crate sha2;
extern crate base64;

use cursive::Cursive;

mod screens;

fn main() {
    let mut siv = Cursive::ncurses();

    screens::main_screen(&mut siv);

    siv.run();
}
