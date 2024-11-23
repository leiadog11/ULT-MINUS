use super::*;

pub mod acmd;
pub mod frame;
pub mod status;
pub mod bigpacman;
pub mod firehydrant;
pub mod trampoline;

const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH : i32 = 0x200000EC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_DOWN_SMASH : i32 = 0x200000ED;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_ITEM_CHOICE : i32 = 0x100000C9;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_KEY_COOLDOWN : i32 = 0x100000CA;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPLE_COOLDOWN : i32 = 0x100000BD;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_MELON_COOLDOWN : i32 = 0x100000BC;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_GALAXIAN_COOLDOWN : i32 = 0x100000BB;
const FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_BELL_COOLDOWN : i32 = 0x100000CB;

// SIDE B BLASTZONE CHECK
unsafe extern "C" fn blastzone_check(agent: &mut L2CAgentBase) { 
    if smash::app::stage::get_stage_id() == 0x0 {
        //Battlefield
        if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x3 {
        //FD
        if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x15B {
        //Small Battlefield
        if PostureModule::pos_x(agent.module_accessor) >= 170.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -170.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 170.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x5F {
        //Smashville
        if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 140.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 140.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x6B {
        //PS2
        if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -50.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -50.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0xF2 {
        //Kalos
        if PostureModule::pos_x(agent.module_accessor) >= 165.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -165.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 165.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x101 {
        //Town and City
        if PostureModule::pos_x(agent.module_accessor) >= 160.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -160.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 160.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -80.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 105.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 105.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -80.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
    else if smash::app::stage::get_stage_id() == 0x169 {
        //Hollow Bastion
        if PostureModule::pos_x(agent.module_accessor) >= 180.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: -180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_x(agent.module_accessor) <= -180.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: 180.0 , y: PostureModule::pos_y(agent.module_accessor), z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) <= -60.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: 130.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
        else if PostureModule::pos_y(agent.module_accessor) >= 130.0 {
            PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: PostureModule::pos_x(agent.module_accessor) , y: -60.0, z: PostureModule::pos_z(agent.module_accessor)});
            AttackModule::clear_all(agent.module_accessor);
            GroundModule::set_collidable(agent.module_accessor, false);
        }
    }
}

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    bigpacman::install();
    firehydrant::install();
    trampoline::install();
}