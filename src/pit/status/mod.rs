use super::*;

mod checkattack;
mod speciallw;

pub fn install() {
    checkattack::install();
    speciallw::install();
}