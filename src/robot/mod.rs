use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

//ROB Fighter Frame :)
unsafe extern "C" fn rob_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
    }
}


pub fn install() {
    Agent::new("robot")
        .on_line(Main, rob_frame)
        .install();
}
