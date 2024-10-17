use std::env;
use dotenv::dotenv;
use rust_decimal::Decimal;
use tokio_postgres::{NoTls, Error, Client, Row};

pub async fn connect_to_db() -> Result<(Client, impl std::future::Future<Output = Result<(), Error>>), Error> {
    dotenv().ok();

    let (client, connection) = tokio_postgres::connect(&env::var("DB_URL").expect("cannot connect db"), NoTls).await?;
    Ok((client, connection))
}

pub async fn create_category_table(client: &Client) -> Result<(), Error> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS Category (
            id SERIAL PRIMARY KEY,
            category TEXT NOT NULL
        )", &[]
    ).await?;
    Ok(())
}

pub async fn create_budget_table(client: &Client) -> Result<(), Error> {
    client.execute("CREATE TABLE IF NOT EXISTS Budget (
        id SERIAL PRIMARY KEY,
        price decimal NOT NULL,
        title TEXT,
        categoryId integer REFERENCES Category
    )", &[]
    ).await?;
    Ok(())
}

pub async fn select_categories(client: &Client) -> Result<Vec<Row>, Error> {
    let rows = client.query("SELECT * FROM Category", &[]).await?;
    Ok(rows)
}

pub async fn insert_category(client: &Client, category: &str) -> Result<u64, Error> {
    let new_row = client.execute("INSERT INTO Category (category) VALUES ($1)",
     &[&category]).await?;
     Ok(new_row)
}

pub async fn select_budgets(client: &Client) -> Result<Vec<Row>, Error> {
    let rows = client.query("SELECT * FROM Budget", &[]).await?;
    Ok(rows)
}

pub async fn select_single_budget(client: &Client, id: i32) -> Result<Row, Error> {
    let row = client.query("SELECT * FROM Budget WHERE id = $1", &[&id]).await?;
    Ok(row[0].clone())
}

pub async fn insert_budget(client: &Client, price:Decimal, category_id: i32, title: &str) -> Result<i32, Error> {
    let new_row = client.query_one("INSERT INTO Budget (price, categoryId, title) VALUES ($1, $2, $3) RETURNING id", 
    &[&price, &category_id, &title]).await?;

    let new_row_id: i32 = new_row.get(0);
    Ok(new_row_id)
}

pub async fn update_budget(client: &Client, id: i32, price:Decimal, category_id: i32, title: &str) ->Result<(), Error> {
    client.query("UPDATE Budget SET price = $1, categoryId = $2, title = $3 WHERE id = $4",
    &[&price, &category_id, &title, &id]).await?;
    Ok(())
}

pub async fn delete_budget(client: &Client, id: i32) -> Result<(), Error> {
    client.query("DELETE FROM Budget WHERE id = $1", &[&id]).await?;
    Ok(())
}

pub async fn select_sum_of_price(client: &Client) -> Result<Row, Error> {
    let row = client.query_one("SELECT SUM(price) FROM Budget", &[]).await?;
    Ok(row)
}