use blimey::aha::AhaRequest;

#[test]
fn can_initialize() -> Result<(), Box<dyn std::error::Error>> {
    let _aha = AhaRequest::new("token", "sub");

    Ok(())
}
