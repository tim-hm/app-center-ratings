use helpers::hooks::{after, before};

use crate::helpers::env::get_server_base_url;

mod helpers;

#[tokio::test]
async fn quick() {
    before().await;

    let url = get_server_base_url();
    let response = reqwest::get(url).await.unwrap();

    println!("response: {response:?}");

    let actual = response.text().await.unwrap();
    let expected = "id-1";

    println!("body: {actual}");
    assert_eq!(actual, expected);

    after().await;
}
