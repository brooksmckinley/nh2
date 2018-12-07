use cursive::Cursive;


use cursive::views::*;

mod callbacks;

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
    let username_box = EditView::new().on_submit(|s, _| { callbacks::submit(s) });

    // Layout
}
