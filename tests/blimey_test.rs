use blimey::aha::AhaRequest;
use mockito::mock;

#[test]
fn can_initialize() -> Result<(), Box<dyn std::error::Error>> {
    let m = mock("GET", "/")
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();

    m.assert();
    let uri = &mockito::server_url();
    println!("{:?}", uri);
    let aha = AhaRequest::with_domain("token", "sub", "mock", "http");
    let _req = aha.get_feature("FEAT-1");

    Ok(())
}
