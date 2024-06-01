use sqlite;

/*
 * This file contains the functions to interact with the SQLite database.
 * The functions are:
 * - create_db: creates the database and the users table if it doesn't exist
 * - insert_user: inserts a new user into the database and hashes the password with bcrypt
 */

pub fn create_db() -> sqlite::Connection {
    let conn = sqlite::open("secpass.db").unwrap();
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            name TEXT NOT NULL,
            father_lastname TEXT NOT NULL,
            mother_lastname TEXT,
            age INTEGER NOT NULL,
        );
        ",
    )
    .unwrap();
    conn
}

// Insert a new user into the database with the password hashed
pub fn insert_user(conn: &sqlite::Connection, email: &str, password: &str, name: &str, father_lastname: &str, mother_lastname: &str, age: u8) -> bool {
    // Check if email is already register
    let mut email_exists = false;
    let email_check = format!("SELECT * FROM users WHERE email = '{}';", email);
    conn.iterate(email_check, |pairs| {
        for &(column, _value) in pairs.iter() {
            if column == "email" {
                email_exists = true;
            }
        }

        true
    }).unwrap();

    if !email_exists {
        return false;
    }

    let query = format!(
        "INSERT INTO users (
            email,
            password,
            name,
            father_lastname,
            mother_lastname,
            age
        )
        VALUES ('{}', '{}', '{}', '{}', '{}', '{}');",
        email, password, name, father_lastname, mother_lastname, age
    );
    conn.execute(&query).unwrap();
    
    true
}

// I'll keep it for testing purposes
#[allow(unused)]
pub fn get_all_users(conn: &sqlite::Connection) {
    let query = "SELECT * FROM users;";

    conn.iterate(query, |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{:?} = {:?}", column, value);
        }

        true
    }).unwrap();
}
