use cursive::views::*;
use cursive::Cursive;

use ::screens::*;

use sha2::Sha512;
use sha2::Digest;

pub fn login(siv: &mut Cursive) {
    login_screen(siv, None, None);
}

pub fn register(siv: &mut Cursive) {
    let layer = siv.pop_layer().unwrap();
    unimplemented!()
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
    
    // Give the layer back with the information it had earlier
    login_screen(siv, Some(username.clone()), Some(password.clone()));

    let mut hasher = Sha512::default();

    hasher.input(password.clone().into_bytes());

    popup_dialog(siv, format!("{:?}, {:?}", &username, ::base64::encode(&hasher.result())));

    }

