use cursive::Cursive;

use cursive::views::*;

use db::User;

pub mod callbacks;

pub fn main_screen(siv: &mut Cursive) {
    // Views
    let title_view = TextView::new("Welcome to the Scorpion Land Nethack server");
    let login_button = Button::new("Login", callbacks::login);
    let register_button = Button::new("Register", callbacks::register);
    let quit_button = Button::new("Quit", callbacks::quit);

    // The layout itself
    let layout = LinearLayout::vertical()
        .child(title_view)
        .child(login_button)
        .child(register_button)
        .child(quit_button);

    siv.pop_layer();
    siv.add_layer(Dialog::around(layout).title("SL Nethack"));
}

pub fn login_screen(siv: &mut Cursive, username: Option<String>, password: Option<String>) {
    // Views
    let username_text = TextView::new("Username");
    let username_box = match username { // Set content to argument
        Some(contents) => EditView::new()
            .content(contents),
        None => EditView::new()
    }.on_submit(|s, _| { callbacks::submit_login(s) });

    let password_text = TextView::new("Password");
    let password_box = match password {
        Some(contents) => EditView::new()
            .content(contents),
        None => EditView::new()
    }.secret().on_submit(|s, _| { callbacks::submit_login(s) });

    // Layout
    let layout = LinearLayout::vertical()
        .child(username_text)
        .child(username_box)
        .child(password_text)
        .child(password_box);

    let dialog = Dialog::around(layout)
        .title("Login")
        .button("Back", main_screen)
        .button("Submit", callbacks::submit_login);

    siv.pop_layer();
    siv.add_layer(dialog);
}

pub fn register_screen(siv: &mut Cursive, username: Option<String>, password: Option<String>) {
    // TODO: Password confirmation
    // Views
    let username_text = TextView::new("Username");
    let username_box = match username { // Set content to argument
        Some(contents) => EditView::new()
            .content(contents),
        None => EditView::new()
    }.on_submit(|s, _| { callbacks::submit_register(s) });

    let password_text = TextView::new("Password");
    let password_box = match password {
        Some(contents) => EditView::new()
            .content(contents),
        None => EditView::new()
    }.secret().on_submit(|s, _| { callbacks::submit_register(s) });

    // Layout
    let layout = LinearLayout::vertical()
        .child(username_text)
        .child(username_box)
        .child(password_text)
        .child(password_box);

    let dialog = Dialog::around(layout)
        .title("Register")
        .button("Back", main_screen)
        .button("Submit", callbacks::submit_register);

    siv.pop_layer();
    siv.add_layer(dialog);
}

pub fn home_screen(siv: &mut Cursive, user: User) {
    // TODO: Change password button
    let text = TextView::new(format!("Welcome back {}!", user.name));
    let play = Button::new("Play", move |s| { callbacks::play(s, user.clone()) });
    let layout = LinearLayout::vertical()
        .child(text)
        .child(play);
    let dialog = Dialog::around(layout)
        .title("Home Screen")
        .button("Quit", callbacks::quit);
    siv.pop_layer();
    siv.add_layer(dialog);
}

pub fn popup_dialog(siv: &mut Cursive, text: String) {
    //let layer = siv.pop_layer();
    let view = TextView::new(text);
    let dialog = Dialog::around(view)
        .button("Ok", move |siv| {
            siv.pop_layer();
        });
    // Put the dialog and previous layer back, but leave the dialog on top
    /*if let Some(l) = layer {
        siv.add_layer(l);
    }*/
    siv.add_layer(dialog);
}

