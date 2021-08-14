use blimey::aha::AhaRequest;
use mockito::mock;

#[async_std::test]
async fn test_get_feature() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/features/FEAT-1")
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url("token", "sub", uri);
    aha.get_feature("FEAT-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_get_release() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/releases/REL-1")
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url("token", "sub", uri);
    aha.get_release("REL-1").await?;

    m.assert();
    Ok(())
}

#[async_std::test]
async fn test_get_product() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/api/v1/products/PROD-1")
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    let uri = &mockito::server_url();
    let aha = AhaRequest::with_url("token", "sub", uri);
    aha.get_product("PROD-1").await?;

    m.assert();
    Ok(())
}
