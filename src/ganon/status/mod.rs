use super::*;

mod attack_hi4;
mod attack_lw4;
mod attack_s4;
mod check_damage;
mod special_n;
mod special_lw;
mod special_hi;
mod special_hi2;

pub fn install() {
    attack_hi4::install();
    attack_lw4::install();
    attack_s4::install();
    check_damage::install();
    special_n::install();
    special_lw::install();
    special_hi::install();
    special_hi2::install();
}