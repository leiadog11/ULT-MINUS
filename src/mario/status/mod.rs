use super::*;

pub mod shrink;
pub mod speciallw;

pub fn install() {
    shrink::install();
    speciallw::install();
}