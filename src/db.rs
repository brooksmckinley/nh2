use rusqlite::Connection;
use rand::Rng;
use sha2::{Sha512, Digest};

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub password_hash: String,
    pub salt: String,
}

#[derive(Debug)]
pub enum RegisterError {
    InvalidName,
    NameTaken,
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

pub fn register_user(name: String, pass: String) -> Result<User, RegisterError> {
    // Validate the name
    if !validate(&name) {
        return Err(RegisterError::InvalidName);
    }
    // Check if the name is taken
    if let Some(_) = get_user(&name) {
        return Err(RegisterError::NameTaken);
    }

    // Now that we're sure that the user is available, register it.

    // Generate the salt
    let mut rng = rand::thread_rng();
    let mut salt: [u8; 64] = [0; 64];
    rng.fill(&mut salt);
    let salt_base64 = ::base64::encode(&salt[..]);

    // Generate the hash
    let hasher = Sha512::default()
        .chain(pass.into_bytes())
        .chain(&salt[..]);
    let hash = ::base64::encode(&hasher.result());

    let user = User {
        name: name,
        password_hash: hash,
        salt: salt_base64,
    };
    
    Ok(user)
}
