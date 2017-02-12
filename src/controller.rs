use Device;

use ssdp::FieldMap;
use ssdp::header::{HeaderMut, Man, MX, ST};
use ssdp::message::SearchRequest;

pub struct Controller {
    devices: Vec<Device>,
}

impl Controller {
    pub fn new() -> Self {
        Controller { devices: vec![] }
    }

    pub fn discover(&mut self) {
        let mut request = SearchRequest::new();
        let search_target = FieldMap::new("urn:schemas-upnp-org:device:ZonePlayer:1").unwrap();

        request.set(Man);
        request.set(MX(10));
        request.set(ST::Target(search_target));

        for (_, src) in request.multicast().unwrap() {
            let device = Device::new(src);
            self.devices.push(device);
        }
    }
}
