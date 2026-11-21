use super::*;

mod check_attack;
mod check_damage;
mod special_n;

pub fn install() {
    check_attack::install();
    check_damage::install();
    special_n::install();
}