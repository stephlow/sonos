use endpoints::DeviceInfo;

mod speaker;
use self::speaker::Speaker;

mod accesory;
use self::accesory::Accesory;

use std::net::SocketAddr;

pub enum Device {
    Accesory(Accesory),
    Speaker(Speaker),
}

impl Device {
    pub fn new(addr: SocketAddr) -> Self {
        let device_info = DeviceInfo::new(addr);

        match device_info.device_type.as_ref() {
            "urn:schemas-upnp-org:device:ZonePlayer:1" => {
                let speaker = Speaker::new();
                Device::Speaker(speaker)
            }
            _ => {
                let accesory = Accesory::new();
                Device::Accesory(accesory)
            }
        }
    }
}
