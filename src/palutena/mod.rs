#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod frame;
pub mod status;
pub mod explosiveflame;
pub mod reflectionboard;
pub mod beam;
pub mod autoaimbullet;
pub mod blackhole;

use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash2::*,
    smash_script::*,
    smashline::*,
    smashline::Priority::*
};

pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_CHARGE_MUL : i32 = 0x4E;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE: i32 = 0x100000BF;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE: i32 = 0x100000CA;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PLANTED: i32 = 0x200000EB;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_ANCHOR_TP: i32 = 0x200000EC;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_UP_B_USED: i32 = 0x200000ED;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_CHARGE: i32 = 0x1EA;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_SHOOT: i32 = 0x1EB;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_CHARGE: i32 = 0x1EC;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_SHOOT: i32 = 0x1ED;
pub static mut BULLET_X_POS : f32 = 0.0; 
pub static mut BULLET_Y_POS : f32 = 0.0; 

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    explosiveflame::install();
    reflectionboard::install();
    beam::install();
    autoaimbullet::install();
    blackhole::install();
}
