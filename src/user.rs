use sqlite;

#[derive(Debug, Clone)]
pub struct User {
    pub id: u8,
    pub email: String,
    pub password: String,
    pub role: Role
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum Role {
    Admin,
    Client,
    Worker
}

impl User {
    pub fn new(email: &str, password: &str) -> User {
        User {
            id: 0,
            email: String::from(email),
            password: String::from(password),
            role: Role::Client
        }
    }

    // Insert a new user into the database with the password hashed
    pub fn insert_user(conn: &sqlite::Connection, user: User) -> bool {
        let role = match user.role {
            Role::Admin => 1,
            Role::Client => 2,
            Role::Worker => 3,
        };

        let mut email_exists = false;
        let check_email = format!(
            "SELECT * FROM users WHERE email = '{}';",
            user.email
        );
        conn.iterate(check_email, |pairs| {
            for &(column, value) in pairs.iter() {
                if column == "email" {
                    email_exists = true;
                }
            }

            true
        }).unwrap();

        if email_exists {
            return false;
        }

        let query = format!(
            "INSERT INTO users (email, password, role)
            VALUES ('{}', '{}', {});",
            user.email, user.password, role
        );

        conn.execute(&query).unwrap();

        true
    }

    // I'll keep it for testing purposes
    /* #[allow(unused)]
    fn get_all_users(conn: &sqlite::Connection) -> Vec<User> {
        let query = "SELECT * FROM users;";

        conn.iterate(query, |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{:?} = {:?}", column, value);
            }

            true
        }).unwrap();
    } */
}
