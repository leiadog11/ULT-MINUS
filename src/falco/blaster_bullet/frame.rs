use super::*;

// OPWF
unsafe extern "C" fn blaster_bullet_frame(weapon: &mut L2CWeaponCommon) {
    unsafe {
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_motion = MotionModule::motion_kind(owner_boma);
        let pos_x = PostureModule::pos_x(weapon.module_accessor);
        let pos_y = PostureModule::pos_y(weapon.module_accessor);

        // BACK AIR LASER?
        /* This is close! It needs to set the speed through hashes
        if owner_motion == hash40("attack_air_b") {
            PostureModule::set_pos_2d(weapon.module_accessor, &Vector2f {x: -pos_x, y: pos_y});
        }
        */
    }
}

pub fn install() {
    Agent::new("falco_blaster_bullet")
        .on_line(Main, blaster_bullet_frame)
        .install();
}
