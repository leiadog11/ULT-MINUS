use super::*;

// OPFF
pub unsafe extern "C" fn wickedweaveleg_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let boma = weapon.module_accessor;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

        // DOWN SMASH
        if MotionModule::motion_kind(boma) == hash40("attack_lw4") {
            if frame >= 10.0 {
                GroundModule::set_collidable(boma, false);
                let pos_y = PostureModule::pos_y(boma);
                let pos_x = PostureModule::pos_x(boma);
                let pos_z = PostureModule::pos_z(boma);

                let mut newPos = Vector3f{x: pos_x, y: pos_y - 1.5, z: pos_z};
                PostureModule::set_pos(boma, &newPos);
            }
        }

        // FORWARD SMASH
        if MotionModule::motion_kind(boma) == hash40("attack_s4_s") {
            if frame >= 13.0 {
                let lr = PostureModule::lr(owner_boma);
                let pos_y = PostureModule::pos_y(boma);
                let pos_x = PostureModule::pos_x(boma);
                let pos_z = PostureModule::pos_z(boma);

                let mut newPos = Vector3f{x: pos_x + 1.0 * lr, y: pos_y, z: pos_z};
                PostureModule::set_pos(boma, &newPos);
            }
        }
    }
}

pub fn install() {
    Agent::new("bayonetta_wickedweaveleg")
        .on_line(Main, wickedweaveleg_frame)
        .install();
}