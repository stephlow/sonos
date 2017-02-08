extern crate ssdp;

mod sonos;
pub use sonos::Sonos;

mod player;
pub use player::Player;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
