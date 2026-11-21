use super::*;

mod attack_hi3;
mod check_attack;
mod special_hi;
mod special_n;

pub fn install() {
    attack_hi3::install();
    check_attack::install();
    special_hi::install();
    special_n::install();
}