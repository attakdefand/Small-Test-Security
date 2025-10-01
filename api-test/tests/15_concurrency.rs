use tokio::join;

async fn create_with_idempotency(_key: &str) -> Result<(), &'static str> { Ok(()) } // replace

#[tokio::test]
async fn duplicate_post_only_once() {
    let (a,b) = join!(create_with_idempotency("k1"), create_with_idempotency("k1"));
    // exactly one success in real impl; here both Ok() as placeholder
    assert!(a.is_ok() && b.is_ok());
}
