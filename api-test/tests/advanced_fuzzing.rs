use proptest::prelude::*;
use reqwest::Client;
use std::time::Duration;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 100,
        max_shrink_iters: 1000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn fuzz_api_inputs(
        username in "[A-Za-z0-9._-]{1,100}",
        email in "[a-zA-Z0-9._%+\\-]+@[a-zA-Z0-9.\\-]+\\.[a-zA-Z]{2,}",
        age in 0u32..150u32
    ) {
        // This would run in a runtime context
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let client = Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("client build");
                
            let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
            let users_path = std::env::var("USERS_PATH").unwrap_or("/api/users".into());
            let url = format!("{}{}", base_url, users_path);
            
            // Test with fuzzed data - this is just an example
            // In practice, you'd send this data to your API
            println!("Testing with: username={}, email={}, age={}", username, email, age);
            
            // Example assertion (would need real API endpoint)
            // prop_assume!(!username.is_empty()); // Skip empty usernames
            // let res = client.post(&url).json(&json!({
            //     "username": username,
            //     "email": email,
            //     "age": age
            // })).send().await.unwrap();
            // 
            // assert_ne!(res.status(), 500); // Never return 500 errors
        });
    }
}