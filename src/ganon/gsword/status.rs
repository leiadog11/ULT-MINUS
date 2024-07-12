use super::*;

/////// REGULAR

// PRE
unsafe extern "C" fn ganon_gsword_regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_AIR.into(), 
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
unsafe extern "C" fn ganon_gsword_regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let life = 300;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    weapon.fastshift(L2CValue::Ptr(ganon_gsword_regular_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn ganon_gsword_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let facing = PostureModule::lr(weapon.module_accessor);
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;

    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);

    // Declare acceleration and max speed
    speed_x = if facing == 1.0 {0.5} else {-0.5};
    let accel_y: f32 = -0.0054; // Adjusted for controlled y movement
    let speed_max_y: f32 = 1.0; // Adjusted max speed for y movement

     // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();

    if status_frame == 1.0 {
        speed_y = 0.5;
    }

    speed_y += accel_y;
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: PostureModule::rot_x(weapon.module_accessor, 0) + 10.0, y: 0.0, z: 0.0}, 0);


    // Set speed
    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);


    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        gsword_remove(weapon);
        return 0.into();
    }

    if weapon.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
        gsword_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn gsword_remove(weapon: &mut smashline::L2CWeaponCommon) {
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
    Agent::new("ganon_gsword")
        .status(Pre, WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR, ganon_gsword_regular_pre)
        .status(Main, WEAPON_GANON_GSWORD_STATUS_KIND_REGULAR, ganon_gsword_regular_main)
        .install();
}