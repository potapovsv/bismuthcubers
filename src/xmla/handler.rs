use crate::xmla::parser;
use std::str;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::Writer;
use std::io::Cursor;

pub async fn xmla_handler(req: actix_web::HttpRequest, body: actix_web::web::Bytes) -> actix_web::HttpResponse {
    let content_type = req
        .headers()
        .get("Content-Type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    if content_type.contains("text/xml") || content_type.contains("application/soap+xml") {
        let xml_str = match str::from_utf8(&body) {
            Ok(s) => s,
            Err(_) => return actix_web::HttpResponse::BadRequest().body("Invalid UTF-8"),
        };

        match parser::parse_xmla(xml_str) {
            Ok(request_type) => {
                let response_xml = match request_type.as_str() {
                    "Discover" => generate_discover_response(),
                    _ => format!("<return><status>OK</status><type>{}</type></return>", request_type),
                };

                actix_web::HttpResponse::Ok()
                    .content_type("text/xml; charset=utf-8")
                    .body(response_xml)
            }
            Err(e) => actix_web::HttpResponse::BadRequest().body(e),
        }
    } else {
        actix_web::HttpResponse::UnsupportedMediaType().body("Unsupported Content-Type")
    }
}

pub fn generate_discover_response() -> String {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // Envelope
    let mut envelope = BytesStart::borrowed_name(b"Envelope");
    envelope.push_attribute(("xmlns", "http://schemas.xmlsoap.org/soap/envelope/"));
    envelope.push_attribute(("xmlns:rs", "urn:schemas-microsoft-com:xml-analysis:rowset"));
    envelope.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    writer.write_event(Event::Start(envelope)).expect("write failed");

    // Body
    let body = BytesStart::borrowed_name(b"Body");
    writer.write_event(Event::Start(body)).expect("write failed");

    // DiscoverResponse
    let mut discover_response = BytesStart::borrowed_name(b"DiscoverResponse");
    discover_response.push_attribute(("xmlns", "urn:schemas-microsoft-com:xml-analysis"));
    writer.write_event(Event::Start(discover_response)).expect("write failed");

    // return
    let ret = BytesStart::borrowed_name(b"return");
    writer.write_event(Event::Start(ret)).expect("write failed");

    // root
    let mut root = BytesStart::borrowed_name(b"root");
    root.push_attribute(("xmlns", "urn:schemas-microsoft-com:xml-analysis:rowset"));
    writer.write_event(Event::Start(root)).expect("write failed");

    // xsd:schema
    let mut schema = BytesStart::borrowed_name(b"xsd:schema");
    schema.push_attribute(("xmlns:xsd", "http://www.w3.org/2001/XMLSchema"));
    writer.write_event(Event::Start(schema)).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"xsd:schema"))).expect("write failed");

    // rowset
    let mut rowset = BytesStart::borrowed_name(b"rowset");
    rowset.push_attribute(("xmlns", "urn:schemas-microsoft-com:xml-analysis:rowset"));
    writer.write_event(Event::Start(rowset)).expect("write failed");

    // row
    let row = BytesStart::borrowed_name(b"row");
    writer.write_event(Event::Start(row)).expect("write failed");

    write_element(&mut writer, b"CATALOG_NAME", "DefaultCatalog");
    write_element(&mut writer, b"CUBE_NAME", "SalesCube");
    write_element(&mut writer, b"CUBE_TYPE", "CUBE");

    writer.write_event(Event::End(BytesEnd::borrowed(b"row"))).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"rowset"))).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"root"))).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"return"))).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"DiscoverResponse"))).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"Body"))).expect("write failed");
    writer.write_event(Event::End(BytesEnd::borrowed(b"Envelope"))).expect("write failed");

    let bytes = writer.into_inner().into_inner();
    String::from_utf8_lossy(&bytes).to_string()
}

fn write_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &[u8],
    value: &str,
) {
    writer
        .write_event(Event::Start(BytesStart::borrowed_name(name)))
        .expect("write failed");
    writer
        .write_event(Event::Text(quick_xml::events::BytesText::from_plain_str(value)))
        .expect("write failed");
    writer
        .write_event(Event::End(BytesEnd::borrowed(name)))
        .expect("write failed");
}