use crate::db;
use bcrypt;

pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum PasswordError {
    TooShort,
    NoUppercase,
    NoLowercase,
    NoNumber,
    NoSpecial,
}

// Function to check if the password meets the requirements
// This function is called in the frontend to check if the password is valid
// If is not valid, it will return an error with the reason and in the frontend
// the user will see the error message and never call the register_user function
pub fn check_password(password: &str) -> Result<(), PasswordError> {
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_number = false;
    let mut has_special = false;

    match password.len() {
        0..=7 => return Err(PasswordError::TooShort),
        _ => {
            for c in password.chars() {
                if c.is_uppercase() {
                    has_uppercase = true;
                } else if c.is_lowercase() {
                    has_lowercase = true;
                } else if c.is_numeric() {
                    has_number = true;
                } else {
                    has_special = true;
                }
            }
        }
    }
    
    let res = has_uppercase && has_lowercase && has_number && has_special;
    if !res {
        if !has_uppercase {
            return Err(PasswordError::NoUppercase);
        } else if !has_lowercase {
            return Err(PasswordError::NoLowercase);
        } else if !has_number {
            return Err(PasswordError::NoNumber);
        } else {
            return Err(PasswordError::NoSpecial);
        }
    }

    Ok(())
}

// Function to register a user in the database
// Here the password will be hashed and that hash will be stored in the database
pub fn register_user(username: &str, password: &str) {
    let user = User::new(username, password);

    let hashed_password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();
    let conn = db::create_db();
    db::insert_user(&conn, &user.username, &hashed_password);
}

// Function to login a user
// Here the password will be hashed and that hash will be compared with the hash stored in the database
#[allow(unused)]
pub fn login_user(username: &str, password: &str) -> bool {
    let user = User::new(username, password);

    let conn = db::create_db();
    let query = format!(
        "SELECT * FROM users WHERE username = '{}';",
        user.username
    );

    let mut found = false;
    conn.iterate(query, |pairs| {
        let mut user = User::new("", "");
        for &(column, value) in pairs.iter() {
            match column {
                "username" => user.username = String::from(value.unwrap()),
                "password" => user.password = String::from(value.unwrap()),
                _ => (),
            }
        }

        if user.username == username {
            // Decrypt the database password
            let hashed_password = bcrypt::verify(&password, &user.password).unwrap();
            if hashed_password {
                found = true;
            }
        }

        true
    }).unwrap();

    found
}

// I'll keep it for testing purposes
#[allow(unused)]
fn get_all_users() {
    let conn = db::create_db();
    db::get_all_users(&conn);
}
