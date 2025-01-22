use super::*;

////////////// GHOST START

// INIT
pub unsafe extern "C" fn pacman_bigpacman_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    //Snap to throw position
    let mut owner_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut article_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let mut offset_add = Vector3f{x:0.0,y:0.0,z:0.0};

    if WorkModule::is_flag(owner, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH) {
        offset_add = Vector3f{x:0.0,y:20.0,z:0.0};
    }
    else if WorkModule::is_flag(owner, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_DOWN_SMASH) {
        offset_add = Vector3f{x:-20.0,y:0.0,z:0.0};
    }
    else {
        offset_add = Vector3f{x:20.0,y:0.0,z:0.0};
    }

    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("throw")}, &mut owner_pos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut article_pos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + owner_pos.x - article_pos.x + (offset_add.x*lr), y: PostureModule::pos_y(owner) + owner_pos.y - (article_pos.y)+ offset_add.y, z: PostureModule::pos_z(owner) + owner_pos.z - article_pos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    0.into()
}

// PRE
unsafe extern "C" fn pacman_bigpacman_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// MAIN
unsafe extern "C" fn pacman_bigpacman_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    //Life
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("start"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(weapon.module_accessor);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);

    if WorkModule::is_flag(owner_boma, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_UP_SMASH) {
        speed_x = 0.0;
        speed_y = 1.0;
    }
    else if WorkModule::is_flag(owner_boma, FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_DOWN_SMASH) {
        speed_x = if lr == 1.0 { -1.0 } else { 1.0 };
        speed_y = 0.0; 
    }
    else { 
        speed_x = if lr == 1.0 { 1.0 } else { -1.0 };
        speed_y = 0.0; 
    }

    // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();
    
    // Set speed
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    weapon.fastshift(L2CValue::Ptr(pacman_bigpacman_start_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn pacman_bigpacman_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    // REFLECTION CHECK
    if (AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 1.2, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        return 0.into();
    }

    if life < 0 {
        bigpacman_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn bigpacman_remove(weapon: &mut smashline::L2CWeaponCommon) {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_misfire"),
        pos,
        &Vector3f{x: 0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("pacman_bigpacman")
        .status(Init, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_START, pacman_bigpacman_start_init)
        .status(Pre, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_START, pacman_bigpacman_start_pre)
        .status(Main, *WEAPON_PACMAN_BIGPACMAN_STATUS_KIND_START, pacman_bigpacman_start_main)
        .install();
}