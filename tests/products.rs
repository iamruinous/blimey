use blimey::aha::AhaRequest;
use mockito::{mock, Matcher};
use serde_json::json;

const TEST_TOKEN: &str = "test_token";
const TEST_SUBDOMAIN: &str = "test_sub";
const BEARER_TOKEN: &str = "Bearer test_token";

#[async_std::test]
async fn test_get_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/products/PROD-1")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.get_product("PROD-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_list_products() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/products")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.list_products(&None).await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_list_products_with_updated_since() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/products?updated_since=2022-08-13T15:15:15Z")
        .match_header("Authorization", BEARER_TOKEN)
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.list_products(&Some("2022-08-13T15:15:15Z".to_string()))
        .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_create_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("POST", "/api/v1/products")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"product":{"name":"newname","prefix":"newprefix","parent_id":"PROD-2","workspace_type":"product_workspace"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.create_product(
        "newname",
        "newprefix",
        &Some("PROD-2".to_string()),
        "product_workspace",
    )
    .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_create_product_without_product_id() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("POST", "/api/v1/products")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"product":{"name":"newname","prefix":"newprefix","workspace_type":"product_workspace"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.create_product("newname", "newprefix", &None, "product_workspace")
        .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/products/PROD-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"product":{"name":"newname","prefix":"newprefix","parent_id":"PROD-2"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_product(
        "PROD-1",
        &Some("newname".to_string()),
        &Some("newprefix".to_string()),
        &Some("PROD-2".to_string()),
    )
    .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_product_without_parent_id() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/products/PROD-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"product":{"name":"newname","prefix":"newprefix"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_product(
        "PROD-1",
        &Some("newname".to_string()),
        &Some("newprefix".to_string()),
        &None,
    )
    .await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_update_product_without_name() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("PUT", "/api/v1/products/PROD-1")
        .match_header("Authorization", BEARER_TOKEN)
        .match_body(Matcher::Json(
            json!({"product":{"prefix":"newprefix","parent_id":"PROD-2"}}),
        ))
        .with_status(204)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url(TEST_TOKEN, TEST_SUBDOMAIN, uri);
    aha.update_product(
        "PROD-1",
        &None,
        &Some("newprefix".to_string()),
        &Some("PROD-2".to_string()),
    )
    .await?;

    m.assert();
    Ok(())
}
