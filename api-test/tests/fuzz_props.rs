mod common;
use common::*;
use proptest::prelude::*;
use http::StatusCode;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn never_500_on_symbol(symbol in "[A-Za-z0-9._-]{0,32}") {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let base = base_url();
            let mut url = base.join(&path("QUOTES_PATH")).unwrap();
            url.set_query(Some(&format!("symbol={}", symbol)));
            let res = client().get(url)
                .header("Authorization", bearer("USER_TOKEN"))
                .send().await.unwrap();
            assert_ne!(res.status(), StatusCode::INTERNAL_SERVER_ERROR, "no 500s for weird inputs");
        });
    }
}
