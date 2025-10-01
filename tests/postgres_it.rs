#![cfg(feature = "dbtests")]

use testcontainers::{clients::Cli, images::postgres::Postgres};
use tokio_postgres::NoTls;

#[tokio::test]
async fn can_query_live_postgres() {
    let docker = Cli::default();
    let node = docker.run(Postgres::default());
    let port = node.get_host_port_ipv4(5432);
    let conn = format!("host=127.0.0.1 port={port} user=postgres password=postgres");

    let (client, connection) = tokio_postgres::connect(&conn, NoTls).await.unwrap();
    tokio::spawn(connection);

    let rows = client.query("SELECT 40 + 2", &[]).await.unwrap();
    let v: i32 = rows[0].get(0);
    assert_eq!(v, 42);
}
