extern crate sonos;

use sonos::Sonos;

fn main() {
    let mut sonos = Sonos::new();
    sonos.discover();
}
