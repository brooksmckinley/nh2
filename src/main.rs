extern crate base64;
extern crate cursive;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate sha2;

use cursive::Cursive;

use db::User;
use screens::Game;

pub mod db;
pub mod game;
pub mod screens;
pub mod theme;

// This kills the functional programmer
static mut EXIT_CONDITION: Option<(bool, Option<User>, Option<Game>)> = None;

fn main() {
    // Initialize the EXIT_CONDITION
    unsafe {
        EXIT_CONDITION = Some((false, None, None));
    }

    let mut siv = Cursive::ncurses().unwrap();
	siv.set_theme(theme::get_theme());
    screens::main_screen(&mut siv);
    siv.run();
    drop(siv);

    loop {
        // Read from the EXIT_CONDITION and return its values.
        let (continue_loop, user, game) = unsafe { EXIT_CONDITION.as_ref().unwrap().clone() };

        if !continue_loop {
            break;
        }

        // Run the game
        if let Some(ref u) = user {
            //game::play_game(u);
            match game.unwrap() {
                Game::Nethack => game::play_game(u),
                Game::Hunt => game::play_hunt(u),
            }
        }

        // Now that the game is over, set the exit condition and put the interface back
        unsafe {
            EXIT_CONDITION = Some((false, user.clone(), None));
        }
        let mut siv = Cursive::ncurses().unwrap();
        if let Some(u) = user {
            screens::home_screen(&mut siv, u);
        }
        // If the user isn't initialized for whatever reason, go back to the main screen
        else {
            screens::main_screen(&mut siv);
        }
        siv.run();
        drop(siv);
    }
}
