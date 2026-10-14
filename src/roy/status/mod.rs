use super::*;

mod checkattack;
mod checkdamage;
mod speciallw;
mod specials;

pub fn install() {
    checkattack::install();
    checkdamage::install();
    speciallw::install();
    specials::install();
}