use super::*;

mod check_attack;
mod check_damage;
mod special_hi;
mod special_lw;
mod special_s;

pub fn install() {
    check_attack::install();
    check_damage::install();
    special_hi::install();
    special_lw::install();
    special_s::install();
}