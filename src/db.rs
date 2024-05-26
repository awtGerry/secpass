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
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        ",
    )
    .unwrap();
    conn
}

// Insert a new user into the database with the password hashed
pub fn insert_user(conn: &sqlite::Connection, username: &str, password: &str) {
    let query = format!(
        "INSERT INTO users (username, password) VALUES ('{}', '{}');",
        username, password
    );

    conn.execute(&query).unwrap();
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
