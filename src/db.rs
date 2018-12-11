use rusqlite::*;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub password_hash: String,
    pub salt: String,
}

fn establish_connection() -> Connection { 
    Connection::open("./nethack.db").unwrap()
}

pub fn get_user(name: &str) -> Option<User> {
    let connection = establish_connection();
    let user = connection.query_row("SELECT * FROM users WHERE name = ?1", &[name], |row| {
        User { 
            name: row.get(1),
            password_hash: row.get(2),
            salt: row.get(3),
        }
    });
    eprintln!("{:?}", user);
    match user {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}
