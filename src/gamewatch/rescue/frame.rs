use super::*;

// OPFF
pub unsafe extern "C" fn rescue_frame(weapon: &mut L2CWeaponCommon) {
    unsafe { 
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_posx = PostureModule::pos_x(owner_boma);
        let owner_posy = PostureModule::pos_y(owner_boma);
        let owner_posz = PostureModule::pos_z(owner_boma);
        let lr = PostureModule::lr(owner_boma);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        let frame = MotionModule::frame(weapon.module_accessor);

        println!("rescue frame");

        if motion_kind == hash40("attack_air_b") {
            println!("rescue back air");
            if frame >= 1.0 && frame <= 13.0 {
                println!("phase 1");
                PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 35.0, y: 0.0, z: 0.0}, 0);
                if lr == 1.0 {
                    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx, y: owner_posy, z: owner_posz});
                }
                else {
                    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx, y: owner_posy, z: owner_posz - 2.0});
                }
            }
            if frame >= 14.0 && frame <= 27.0 {
                println!("phase 2");
                PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 35.0, y: 0.0, z: 0.0}, 0);
                if lr == 1.0 {
                    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx, y: owner_posy, z: owner_posz});
                }
                else {
                    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx, y: owner_posy, z: owner_posz - 2.0});
                }
            }
            if frame >= 28.0 && frame <= 42.0 {
                println!("phase 3");
                PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: 35.0, y: 0.0, z: 0.0}, 0);
                if lr == 1.0 {
                    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx, y: owner_posy, z: owner_posz});
                }
                else {
                    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_posx, y: owner_posy, z: owner_posz - 2.0});
                }
            }
        }
    }
}

pub fn install() {
    Agent::new("gamewatch_rescue")
        .on_line(Main, rescue_frame)
        .install();
}