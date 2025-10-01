//! Postgres integration test using testcontainers 0.22 API.
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

#[tokio::test]
async fn postgres_container_smoke() {
    // Start Docker client
    let container = postgres::Postgres::default().start().await.unwrap();

    // Map host port for 5432
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let url = format!("postgres://postgres:[email protected]:{}/postgres", port);

    // Connect with tokio-postgres
    let (client, connection) =
        tokio_postgres::connect(&url, tokio_postgres::NoTls)
            .await
            .expect("connect to postgres");

    // Drive the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("postgres connection error: {e}");
        }
    });

    // Simple query
    let row = client.query_one("SELECT 1", &[]).await.expect("select 1");
    let one: i32 = row.get(0);
    assert_eq!(one, 1);
}