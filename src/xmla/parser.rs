use quick_xml::Reader;
use quick_xml::events::Event;
use std::str;

// Простой парсер команды XMLA
pub fn parse_xmla(xml_str: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml_str);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name().as_ref();
                if name == b"Discover" {
                    return Ok("Discover".to_string());
                } else if name == b"Execute" {
                    return Ok("Execute".to_string());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {:?}", e)),
            _ => {}
        }
        buf.clear();
    }

    Err("Unknown XMLA command".to_string())
}