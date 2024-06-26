use sqlite;

/*
 * This file contains the functions to interact with the SQLite database.
 * The functions are:
 * - create_db: creates the database and the users table if it doesn't exist
 * - insert_user: inserts a new user into the database and hashes the password with bcrypt
 */

pub fn create_db() -> sqlite::Connection {
    let conn = sqlite::open("secpass.db").unwrap();
    create_tables(&conn).unwrap();
    conn
}

fn create_tables(conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            role INTEGER NOT NULL,
            FOREIGN KEY (role) REFERENCES roles(id)
        );

        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL,
            quantity INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY,
            role TEXT NOT NULL
        );
        ",
    )
}
