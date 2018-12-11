use rusqlite::*;

pub struct User {
    pub name: String,
    pub password_hash: String,
    pub salt: String,
}

fn establish_connection() -> Connection { 
    Connection::open("./nethack.db").unwrap()
}

//pub fn get_user(name: &str) -> User {
//}
