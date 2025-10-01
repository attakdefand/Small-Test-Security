/// A simple advanced test that doesn't require any special features
#[tokio::test]
#[ignore = "enable for simple advanced test"]
async fn simple_advanced_test() {
    println!("Running a simple advanced test...");
    
    // This test doesn't require any special dependencies
    // It just demonstrates how to structure advanced tests
    
    let test_data = vec![1, 2, 3, 4, 5];
    
    for (index, value) in test_data.iter().enumerate() {
        println!("Processing test data item {}: {}", index, value);
        // Simulate some async work
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    println!("✅ Simple advanced test completed!");
}