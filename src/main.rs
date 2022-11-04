extern crate postgres;
// use postgres::error::Error;

use dotenv::dotenv;
use postgres::{Client, NoTls};

fn main() {
    dotenv().ok();
    let postgres_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
    let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
    let postgres_password =
        std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set.");
    let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set.");

    let postgres_url =
        format!("postgresql://{postgres_user}:{postgres_password}@{postgres_host}/{postgres_db}");

    // let dsn = "postgresql://alex:password@postgres/account_development";
    let mut client = match Client::connect(&postgres_url, NoTls) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection error: {}", e);
            return;
        }
    };

    //     match client.batch_execute(
    //         "\
    //     CREATE TABLE ausers (
    //         id SERIAL PRIMARY KEY,
    //         name TEXT NOT NULL,
    //         email TEXT NOT NULL,
    //         age INTEGER NOT NULL
    //     )
    // ",
    //     ) {
    //         Ok(_) => println!("Table created"),
    //         Err(e) => {
    //             println!("Table creation error: {}", e);
    //             return;
    //         }
    //     }

    let name = "James";
    let email = "james@test.com";
    let age = 26;

    match client.execute(
        "INSERT INTO ausers (name, email, age) VALUES ($1, $2, $3)",
        &[&name, &email, &age],
    ) {
        Ok(_) => println!("User created"),
        Err(e) => {
            println!("User creation error: {}", e);
            return;
        }
    }

    match client.query("SELECT id, name, email, age FROM ausers", &[]) {
        Ok(rows) => {
            for row in rows {
                let id: i32 = row.get(0);
                let name: &str = row.get(1);
                let email: &str = row.get(2);
                let age: i32 = row.get(3);
                println!("Found user: {} {} {} {}", id, name, email, age);
            }
        }
        Err(e) => {
            println!("Query error: {}", e);
            return;
        }
    }

    println!("Hello, world!");
}

// pub fn read_users(
//     db: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
// ) -> Result<Vec<User>, Error> {
//     let statement = db.prepare("select * from users")?;

//     let users: Vec<User> = db
//         .query(&statement, &[])?
//         .iter()
//         .map(|row| {
//             let phone: String = row.get("phone");
//             let email: String = row.get("email");
//             User { phone, email }
//         })
//         .collect();
//     Ok(users)
// }
