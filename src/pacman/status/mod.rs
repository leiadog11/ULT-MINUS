use super::*;

mod attack_dash;
mod check_damage;
mod special_n;

pub fn install() {
    attack_dash::install();
    check_damage::install();
    special_n::install();
}