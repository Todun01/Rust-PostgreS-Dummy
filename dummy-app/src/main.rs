use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Load .env variables

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Connect to PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("✅ Connected to PostgreSQL!");

    // Sample signup data (simulate input from a user)
    let username = "john_doe";
    let email = "john@example.com";
    let password = "securePassword123";

    // Hash the password
    let hashed = hash(password, DEFAULT_COST).expect("Failed to hash password");

    // Insert user into the database
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        username,
        email,
        hashed
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => println!("🎉 User created successfully!"),
        Err(e) => println!("❌ Failed to create user: {}", e),
    }

    // Optional: simple query
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;

    println!("There are {} users.", row.0);
    Ok(())
}
