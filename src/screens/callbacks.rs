use cursive::views::*;
use cursive::Cursive;

use EXIT_CONDITION;

use screens;
use db;
use db::User;
use game;

use sha2::Sha512;
use sha2::Digest;

pub fn login(siv: &mut Cursive) {
    screens::login_screen(siv, None, None);
}

pub fn register(siv: &mut Cursive) {
    screens::register_screen(siv, None, None);
}

pub fn quit(siv: &mut Cursive) {
    siv.quit();
}

pub fn submit_login(siv: &mut Cursive) {
    // Get the views in the layer
    let layer = siv.pop_layer().unwrap();
    let dialog: &Dialog = layer.as_any().downcast_ref().unwrap();
    let layout: &LinearLayout = dialog.get_content().as_any().downcast_ref().unwrap();
    let username_box: &EditView = layout.get_child(1).unwrap().as_any().downcast_ref().unwrap();
    let password_box: &EditView = layout.get_child(3).unwrap().as_any().downcast_ref().unwrap();

    // Grab the info out of the views
    let username = username_box.get_content().to_string();
    let password = password_box.get_content().to_string();

    // Now that we're done, give the screen back
    screens::login_screen(siv, Some(username.clone()), Some(password.clone()));

    // Attempt to grab a user out of the database
    let user_option = db::get_user(&username);
    // If the user exists, validate the password
    if let Some(user) = user_option {
        // TODO: Validate
        //let mut hasher = Sha512::default();
        //hasher.input(password.clone().into_bytes());
        let hasher = Sha512::default()
            .chain(password.clone().into_bytes())
            .chain(::base64::decode(&user.salt).unwrap());
        let hash = ::base64::encode(&hasher.result());
        if hash == user.password_hash {
            //screens::popup_dialog(siv, "Correct password".to_string());
            screens::home_screen(siv, user);
        }
        else {
            screens::popup_dialog(siv, "Incorrect password".to_string());
        }
    }
    else {
        screens::popup_dialog(siv, "Invalid user.".to_string());
    }
}

pub fn submit_register(siv: &mut Cursive) {
    // Get the views in the layer
    let layer = siv.pop_layer().unwrap();
    let dialog: &Dialog = layer.as_any().downcast_ref().unwrap();
    let layout: &LinearLayout = dialog.get_content().as_any().downcast_ref().unwrap();
    let username_box: &EditView = layout.get_child(1).unwrap().as_any().downcast_ref().unwrap();
    let password_box: &EditView = layout.get_child(3).unwrap().as_any().downcast_ref().unwrap();

    // Grab the info out of the views
    let username = username_box.get_content().to_string();
    let password = password_box.get_content().to_string();

    // Now that we're done, give the screen back
    screens::register_screen(siv, Some(username.clone()), Some(password.clone()));

    // PASTE DONE
    // try and register the user
    let res = db::register_user(username, password);
    if let Ok(user) = res {
        game::generate_skeleton(&user);
        screens::home_screen(siv, user);
        screens::popup_dialog(siv, "Registration successful!".to_string());
    }
    else {
        screens::popup_dialog(siv, format!("{:?}", res));
    }
}

pub fn play(siv: &mut Cursive, user: User) {
    // set the EXIT_CONDITION
    unsafe { EXIT_CONDITION = Some((true, Some(user))) }
    // quit the siv and let the main function handle the rest
    siv.quit();
}
