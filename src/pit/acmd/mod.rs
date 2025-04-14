use super::*;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;

static mut SPECIAL_LW_ACTIVE: [bool; 8] = [false; 8];
static mut SPECIAL_LW_TIMER: [i32; 8] = [0; 8];

pub fn install() {
    aerials::install();
    ground::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
}