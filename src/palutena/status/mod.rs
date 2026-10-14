use super::*;

mod checkdamage;
mod specialn;
mod specials;
mod speciallw;
mod specialhi;

pub fn install() {
    checkdamage::install();
    specialn::install();
    speciallw::install();
    specials::install();
    specialhi::install();
}