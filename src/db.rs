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
            name: row.get(0),
            password_hash: row.get(1),
            salt: row.get(2),
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

    // Make sure the name isn't too long
    if name.len() > 16 {
        return Err(RegisterError::InvalidName);
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

    // SQL that bby in
    let connection = establish_connection();
    connection.execute("INSERT INTO users VALUES(?1, ?2, ?3)", &[&user.name, &user.password_hash, &user.salt]).unwrap();

    Ok(user)
}
