use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod explosiveflame;
pub mod reflectionboard;
pub mod beam;
pub mod autoaimbullet;
pub mod blackhole;

const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_CHARGE: i32 = 0x1EA;
const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_N_SHOOT: i32 = 0x1EB;
const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_CHARGE: i32 = 0x1EC;
const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_S_SHOOT: i32 = 0x1ED;

static mut ANCHOR_PLANTED: [bool; 8] = [false; 8];
static mut UP_B_USED: [bool; 8] = [false; 8];
static mut MEGA_LASER_CHARGE: [i32; 8] = [0; 8];
static mut BLACKHOLE_CHARGE: [i32; 8] = [0; 8];
static mut CHARGE_MUL: [f32; 8] = [1.0; 8];
static mut BULLET_X_POS: [f32; 8] = [0.0; 8];
static mut BULLET_Y_POS: [f32; 8] = [0.0; 8];

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
