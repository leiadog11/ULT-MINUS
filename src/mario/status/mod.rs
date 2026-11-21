use super::*;

mod check_damage;
mod shrink;
mod special_lw;

pub fn install() {
    check_damage::install();
    shrink::install();
    special_lw::install();
}