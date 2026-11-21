use super::*;

mod check_attack;
pub mod special_s;

pub fn install() {
    check_attack::install();
    special_s::install();
}