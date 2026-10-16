use super::*;

mod checkattack;
mod checkdamage;
mod specialhi;
mod speciallw;
mod specials;

pub fn install() {
    checkattack::install();
    checkdamage::install();
    specialhi::install();
    speciallw::install();
    specials::install();
}