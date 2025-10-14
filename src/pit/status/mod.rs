use super::*;

mod checkattack;
mod speciallw;
mod specialhi;

pub fn install() {
    checkattack::install();
    speciallw::install();
    specialhi::install();
}