use std::io::Read;

pub mod message;
pub use message::Message;

extern crate curl;
use curl::easy::{Easy as Curl, List as Head};


pub fn post(url: String, message: &Message) -> Result<(), curl::Error> {
    let head = create_head();
    let payload = create_pyload(&message);

    let mut curl = Curl::new();
    curl.url(url.as_ref()).unwrap();
    curl.http_headers(head).unwrap();
    curl.post(true).unwrap();
    curl.post_field_size(payload.as_bytes().len() as u64).unwrap();

    let mut transfer = curl.transfer();
    transfer.read_function(|buf| {
        Ok(payload.as_bytes().read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform()
}


fn create_head() -> Head {
    let mut head = Head::new();
    head.append("Application: text/json").unwrap();
    head
}

fn create_pyload(message: &Message) -> String {
    format!(r#"payload={}"#, message.to_json())
}
