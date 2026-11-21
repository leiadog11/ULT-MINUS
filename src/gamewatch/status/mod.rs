use super::*;

mod check_attack;
mod check_damage;
mod special_hi;

pub fn install() {
    check_attack::install();
    check_damage::install();
    special_hi::install();
}