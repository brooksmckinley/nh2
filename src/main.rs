extern crate cursive;

use cursive::Cursive;

mod screens;

fn main() {
    let mut siv = Cursive::ncurses();

    screens::main_screen(&mut siv);

    siv.run();
}
