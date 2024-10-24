use super::*;

mod specialn;
mod specials;
mod speciallw;
mod specialhi;

pub fn install() {
    specialn::install();
    speciallw::install();
    specials::install();
    specialhi::install();
}