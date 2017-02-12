use std::net::SocketAddr;
use hyper::Client;
use std::io::Read;

use xml::reader::{EventReader, XmlEvent};

const PORT: u32 = 1400;

pub struct DeviceInfo {
    pub device_type: String,
    pub device_name: String,
    pub room_name: String,
}

impl DeviceInfo {
    pub fn new(addr: SocketAddr) -> Self {
        let endpoint = format!("http://{}:{}/xml/device_description.xml", addr.ip(), PORT);
        let client = Client::new();
        let mut res = client.get(&endpoint).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let parser = EventReader::from_str(&body);

        let mut device_type = String::new();
        let mut device_name = String::new();
        let mut room_name = String::new();

        let mut prev = String::new();

        let mut it = parser.into_iter();
        loop {
            match it.next() {
                Some(x) => {
                    let event = x.unwrap();
                    match event {
                        XmlEvent::StartElement { name, .. } => {
                            prev = name.local_name.to_owned();
                        }
                        XmlEvent::Characters(content) => {
                            match prev.as_ref() {
                                "deviceType" => device_type = content.to_owned(),
                                "modelName" => device_name = content.to_owned(),
                                "roomName" => room_name = content.to_owned(),
                                _ => continue,
                            }
                        }
                        XmlEvent::EndDocument => break,
                        _ => continue,
                    }
                }
                None => break,
            }
        }

        DeviceInfo {
            device_type: device_type,
            device_name: device_name,
            room_name: room_name,
        }
    }
}
