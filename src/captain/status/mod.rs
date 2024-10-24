use super::*;

mod checkattack;
mod specialn;
mod specialhi;
mod speciallw;
mod speciallw2;

pub fn install() {
    checkattack::install();
    specialn::install();
    specialhi::install();
    speciallw::install();
    speciallw2::install();
}