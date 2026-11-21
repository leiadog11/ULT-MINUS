use super::*;

mod check_damage;
mod special_lw;
mod special_s;

pub fn install() {
    check_damage::install();
    special_lw::install();
    special_s::install();
}