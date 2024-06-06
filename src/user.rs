use crate::db;

use sqlite;

pub struct User {
    pub id: u8,
    pub username: String,
    pub password: String,
    pub role: Role
}

pub enum Role {
    Admin,
    Client,
    Worker
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            id: 0,
            username: String::from(username),
            password: String::from(password),
            role: Role::Client
        }
    }

    // Insert a new user into the database with the password hashed
    pub fn insert_user(conn: &sqlite::Connection, user: User) {
        let role = match user.role {
            Role::Admin => String::from("admin"),
            Role::Client => String::from("client"),
            Role::Worker => String::from("worker")
        };

        let query = format!(
            "INSERT INTO users (username, password, role)
            VALUES ('{}', '{}', {});",
            user.username, user.password, role
        );

        conn.execute(&query).unwrap();
    }

    // I'll keep it for testing purposes
    #[allow(unused)]
    fn get_all_users(conn: &sqlite::Connection) -> Vec<User> {
        let query = "SELECT * FROM users;";

        conn.iterate(query, |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{:?} = {:?}", column, value);
            }

            true
        }).unwrap();
    }
}
