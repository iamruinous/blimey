use blimey::aha::AhaRequest;
use mockito::{mock, Matcher};
use serde_json::json;

const TEST_TOKEN: &str = "test_token";
const TEST_SUBDOMAIN: &str = "test_sub";
const BEARER_TOKEN: &str = "Bearer test_token";

#[async_std::test]
async fn test_get_release() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/releases/REL-1")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.get_release("REL-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_list_releases_for_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/products/PROD-1/releases")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.list_releases_for_product("PROD-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_release_for_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/products/PROD-1/releases/REL-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"release":{"name":"newname","parent_id":"PROD-2"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_release_for_product(
        "PROD-1",
        "REL-1",
        &Some("newname".to_string()),
        &Some("PROD-2".to_string()),
    )
    .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_release_for_product_without_name() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/products/PROD-1/releases/REL-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"release":{"parent_id":"PROD-2"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_release_for_product(
        "PROD-1",
        "REL-1",
        &None,
        &Some("PROD-2".to_string()),
    )
    .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_release_for_product_without_parent_id() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/products/PROD-1/releases/REL-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"release":{"name":"newname"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_release_for_product(
        "PROD-1",
        "REL-1",
        &Some("newname".to_string()),
        &None,
    )
    .await?;

    m.assert();
    Ok(())
}
