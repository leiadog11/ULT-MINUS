use super::*;

unsafe extern "C" fn bike_frame(weapon: &mut L2CWeaponCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let lr = PostureModule::lr(boma);
        let xpos = ControlModule::get_stick_x(boma);
        let situation_kind = StatusModule::situation_kind(boma);

        // TURN BIKE IN AIR
        if situation_kind == *SITUATION_KIND_AIR { 
            if lr == 1.0 && xpos < -0.5 { 
                macros::REVERSE_LR(weapon);
            }
            else if lr == -1.0 {
                if xpos > 0.5 {
                    macros::REVERSE_LR(weapon);
                }
            }
        }   

        // BIKE JUMP
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(boma, WEAPON_WARIO_WARIOBIKE_STATUS_WORK_INT_BIKE_JUMP) != 0 {
            for _ in 0..10 {
                PostureModule::set_pos_2d(boma, &Vector2f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) + 0.5});
            }
            WorkModule::dec_int(boma, WEAPON_WARIO_WARIOBIKE_STATUS_WORK_INT_BIKE_JUMP);
        }
    }
}

pub fn install() {
    Agent::new("wario_wariobike")
        .on_line(Main, bike_frame)
    .install();
}