use super::*;

mod checkattack;
mod specials;

pub fn install() {
    checkattack::install();
    specials::install();
}