use super::*;

mod checkattack;
mod speciallw;
mod specialn;

pub fn install() {
    checkattack::install();
    speciallw::install();
    specialn::install();
}