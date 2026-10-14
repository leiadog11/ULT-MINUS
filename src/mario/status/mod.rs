use super::*;

mod checkdamage;
mod shrink;
mod speciallw;

pub fn install() {
    checkdamage::install();
    shrink::install();
    speciallw::install();
}