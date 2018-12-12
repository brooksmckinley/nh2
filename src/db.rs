use rusqlite::Connection;

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub password_hash: String,
    pub salt: String,
}

#[derive(Debug)]
pub enum RegisterError {
    InvalidName,
    Yeet,
}

fn establish_connection() -> Connection { 
    Connection::open("./nethack.db").unwrap()
}

pub fn validate(s: &str) -> bool {
    use regex::Regex;
    let regex = Regex::new("^[0-9a-zA-Z]+$").unwrap();
    regex.is_match(s)
}

pub fn get_user(name: &str) -> Option<User> {
    // Validate the name
    if !validate(name) {
        return None;
    }
    let connection = establish_connection();
    let user = connection.query_row("SELECT * FROM users WHERE name = ?1", &[name], |row| {
        User { 
            name: row.get(1),
            password_hash: row.get(2),
            salt: row.get(3),
        }
    });
    match user {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}

pub fn register_user(name: String, pass: String) -> Result<(), RegisterError> {
    // Validate the name
    if !validate(&name) {
        return Err(RegisterError::InvalidName);
    }
    Ok(())
}
