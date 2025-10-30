use super::*;

mod checkattack;
mod speciallw;
mod specials;

pub fn install() {
    checkattack::install();
    speciallw::install();
    specials::install();
}