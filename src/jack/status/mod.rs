use super::*;

mod checkattack;
pub mod specials;

pub fn install() {
    checkattack::install();
    specials::install();
}