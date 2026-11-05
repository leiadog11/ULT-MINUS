use super::*;

/////////// FIREBALL START

// PRE
unsafe extern "C" fn luigi_fireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_AIR.into(), 
        GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    
    return 0.into();
}

// MAIN
unsafe extern "C" fn luigi_fireball_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue { 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);

    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ENTRY_ID = get_entry_id(owner_boma);
    let mut life = 75;

    if MISFIRE_SPECIAL_N[ENTRY_ID] {
        life = 250;
        PostureModule::set_scale(weapon.module_accessor, 2.5, false);
        WorkModule::on_flag(weapon.module_accessor, WEAPON_LUIGI_FIREBALL_IS_MISFIRE);
    }

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    weapon.fastshift(L2CValue::Ptr(luigi_fireball_start_main_loop as *const () as _))
}

// MAIN LOOP
unsafe extern "C" fn luigi_fireball_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    let mut speed_x = 1.25 * lr;
    let mut rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
    let mut rot_y = PostureModule::rot_y(weapon.module_accessor, 0);
    let mut rot_z = PostureModule::rot_z(weapon.module_accessor, 0);

    if WorkModule::is_flag(weapon.module_accessor, WEAPON_LUIGI_FIREBALL_IS_MISFIRE) {
        rot_y += 15.0;
        speed_x = 0.5 * lr;
    } else {
        rot_x += 30.0;
    }

    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: rot_x, y: rot_y, z: rot_z}, 0);

    // REFLECTION CHECK
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: -speed_x, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        return 0.into();
    }

    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if life < 0 {
        fireball_remove(weapon);
        return 0.into();
    }

    return 0.into();
}

// REMOVE
pub unsafe extern "C" fn fireball_remove(weapon: &mut smashline::L2CWeaponCommon) {
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
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 1.0, 0.5);

    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install() {
    Agent::new("luigi_fireball")
        .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_pre)
        .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_main)
    .install();
}