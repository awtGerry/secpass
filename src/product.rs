#[derive(Debug, Clone)]
pub struct Product {
    pub id: u8,
    pub name: String,
    pub price: f32,
    pub quantity: u16
}

impl Product {
    pub fn new(name: String, price: f32, quantity: u16) -> Product {
        Product {
            id: 0,
            name,
            price,
            quantity
        }
    }

    pub fn new_with_id(id: u8, name: String, price: f32, quantity: u16) -> Product {
        Product {
            id,
            name,
            price,
            quantity
        }
    }

    // Insert new product into the database
    pub fn insert_product(conn: &sqlite::Connection, product: Product) {
        let query = format!(
            "INSERT INTO products (name, price, quantity)
            VALUES ('{}', {}, {});",
            product.name, product.price, product.quantity
        );

        conn.execute(&query).unwrap();
    }

    pub fn update_product(conn: &sqlite::Connection, product: Product) {
        let query = format!(
            "UPDATE products
            SET name = '{}', price = {}, quantity = {}
            WHERE id = {};",
            product.name, product.price, product.quantity, product.id
        );

        conn.execute(&query).unwrap();
    }

    pub fn delete_product(conn: &sqlite::Connection, id: u8) {
        let query = format!("DELETE FROM products WHERE id = {};", id);
        conn.execute(&query).unwrap();
    }

    // Get all products from the database
    pub fn get_all_products(conn: &sqlite::Connection) -> Vec<Product> {
        let query = "SELECT * FROM products;";

        let mut products = Vec::new();
        conn.iterate(query, |pairs| {
            let mut product = Product {
                id: 0,
                name: String::new(),
                price: 0.0,
                quantity: 0
            };

            for &(column, value) in pairs.iter() {
                match column {
                    "id" => product.id = value.unwrap().parse().unwrap(),
                    "name" => product.name = String::from(value.unwrap()),
                    "price" => product.price = value.unwrap().parse().unwrap(),
                    "quantity" => product.quantity = value.unwrap().parse().unwrap(),
                    _ => (),
                }
            }

            products.push(product);
            true
        }).unwrap();

        products
    }
}
