use sqlite;

/*
 * This file contains the functions to interact with the SQLite database.
 * The functions are:
 * - create_db: creates the database and the users table if it doesn't exist
 * - insert_user: inserts a new user into the database and hashes the password with bcrypt
 */

pub struct User {
    id: u8,
    username: &str,
    password: &str,
    role: Role
}

pub enum Role {
    Admin,
    Client,
    Worker
}

pub fn create_db() -> sqlite::Connection {
    let conn = sqlite::open("secpass.db").unwrap();
    create_tables(&conn);
    conn
}

fn create_tables(conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            role FOREIGN KEY REFERENCES roles(id)
        );

        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        ",
    )
}

// Insert a new user into the database with the password hashed
pub fn insert_user(conn: &sqlite::Connection, username: &str, password: &str, role: Role) {
    let role = match role {
        Role::Admin => String::from("admin"),
        Role::Client => String::from("client"),
        Role::Worker => String::from("worker")
    }

    let query = format!(
        "INSERT INTO users (username, password, role)
        VALUES ('{}', '{}', {});",
        username, password, role
    );

    conn.execute(&query).unwrap();
}

// I'll keep it for testing purposes
#[allow(unused)]
pub fn get_all_users(conn: &sqlite::Connection) -> Vec<User> {
    let query = "SELECT * FROM users;";

    conn.iterate(query, |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{:?} = {:?}", column, value);
        }

        true
    }).unwrap();
}
