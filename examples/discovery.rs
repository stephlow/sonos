extern crate sonos;

use sonos::Controller;

fn main() {
    let mut sonos = Controller::new();
    sonos.discover();
}
