use tracing::info;

pub fn parse_xmla(body: &[u8]) -> Result<String, String> {
    let xml_str = std::str::from_utf8(body).map_err(|_| "Invalid UTF-8".to_string())?;
    info!(xml_str);
    if xml_str.contains("<Discover ") {
        info!("Discover is Ok");
        Ok("Discover".to_string())
    } else if xml_str.contains("<Execute ") {
        Ok("Execute".to_string())
    } else {
        info!(xml_str);
        Err("Unknown XMLA command".to_string())
    }
}