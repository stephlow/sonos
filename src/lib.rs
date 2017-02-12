extern crate ssdp;
extern crate hyper;
extern crate xml;

mod endpoints;

mod devices;
use devices::Device;

mod controller;
pub use controller::Controller;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
