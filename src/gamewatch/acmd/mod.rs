use super::*;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;



pub fn install() {
    aerials::install();
    ground::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
}