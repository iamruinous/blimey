use blimey::aha::AhaRequest;
use mockito::{mock, Matcher};
use serde_json::json;

const TEST_TOKEN: &str = "test_token";
const TEST_SUBDOMAIN: &str = "test_sub";
const BEARER_TOKEN: &str = "Bearer test_token";

#[async_std::test]
async fn test_get_feature() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/features/FEAT-1")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.get_feature("FEAT-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_list_features_for_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/products/PROD-1/features")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.list_features_for_product("PROD-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_feature() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/features/FEAT-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"feature":{"name":"newname","start_date":"2022-08-13","due_date":"2022-08-13"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_feature(
        "FEAT-1",
        &Some("newname".to_string()),
        &Some("2022-08-13".to_string()),
        &Some("2022-08-13".to_string()),
    )
    .await?;

    m.assert();
    Ok(())
}
