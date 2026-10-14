use super::*;

mod checkattack;
mod checkdamage;
mod specials;

pub fn install() {
    checkattack::install();
    checkdamage::install();
    specials::install();
}