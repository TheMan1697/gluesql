#[cfg(feature = "sled-storage")]
mod linking_table {
    use gluesql::prelude::{Glue, SledStorage};

    pub async fn run() {
        let storage = SledStorage::new("data/linking-table").unwrap();
        let mut glue = Glue::new(storage);

        let sqls = [
            "CREATE TABLE Customers (id INTEGER, name TEXT);",
            "INSERT INTO Customers VALUES (1, 'John');",
            "INSERT INTO Customers VALUES (2, 'Robert');",
            "CREATE TABLE Orders (id INTEGER, customer_id INTEGER, product TEXT);",
            "INSERT INTO Orders VALUES (100, 1, 'Apples');",
            "INSERT INTO Orders VALUES (101, 2, 'Bananas');",
            "SELECT Customers.name, Orders.product FROM Customers JOIN Orders ON Customers.id = Orders.customer_id;",
        ];

        for sql in sqls {
            glue.execute(sql).await.unwrap();
        }
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    futures::executor::block_on(linking_tables::run());
}
