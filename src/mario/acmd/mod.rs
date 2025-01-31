use super::*;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;

static mut ICEBALL: [bool; 8] = [false; 8];
static mut SHRUNK: [bool; 8] = [false; 8];
static mut FORWARD_SMASH_CHARGE: [f32; 8] = [0.0; 8];
static mut FORWARD_AIR_CHARGE: [f32; 8] = [0.0; 8];

pub fn install() {
    aerials::install();
    ground::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
}