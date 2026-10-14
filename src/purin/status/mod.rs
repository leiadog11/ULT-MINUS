use super::*;

mod checkattack;
mod checkdamage;
mod specialn;

pub fn install() {
    checkattack::install();
    checkdamage::install();
    specialn::install();
}