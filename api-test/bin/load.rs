use goose::prelude::*;
async fn get_health(user: &mut GooseUser) -> TransactionResult { user.get("/health").await?; Ok(()) }
#[tokio::main]
async fn main() -> GooseResult<()> {
  GooseAttack::initialize()?
    .register_scenario(scenario!("health").register_transaction(transaction!(get_health)))
    .set_host(std::env::var("BASE_URL").unwrap_or("http://127.0.0.1:8080".into()))
    .execute().await?;
  Ok(())
}
