use crate::db;
use crate::mfa;
use bcrypt;

use crate::user;
use crate::user::User;

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
pub fn register_user(conn: &sqlite::Connection, email: &str, password: &str) {
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
    let user = User::new(email, &hashed_password);
    user::User::insert_user(conn, user);
}

// Function to login a user
// Here the password will be hashed and that hash will be compared with the hash stored in the database
#[allow(unused)]
pub fn login_user(email: &str, password: &str) -> bool {
    let user = User::new(email, password);

    let conn = db::create_db();
    let query = format!(
        "SELECT * FROM users WHERE email = '{}';",
        user.email
    );

    let mut found = false;
    conn.iterate(query, |pairs| {
        let mut user = User::new("", "");
        for &(column, value) in pairs.iter() {
            match column {
                "email" => user.email = String::from(value.unwrap()),
                "password" => user.password = String::from(value.unwrap()),
                _ => (),
            }
        }

        if user.email == email {
            // Decrypt the database password
            let hashed_password = bcrypt::verify(&password, &user.password).unwrap();
            if hashed_password {
                found = true;
                mfa::Mfa::new(user.email.clone()).send();
            }
        }

        true
    }).unwrap();

    found
}
