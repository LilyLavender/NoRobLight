use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40,
    },
    smash_script::*,
    smashline::{*, Priority::*},
    skyline::hooks::{getRegionAddress, Region}
};

const FIGHTER_ROBOT_INSTANCE_WORK_ID_INT_LAMP_EFEFCT_ID : i32 = 0x100000be;

unsafe extern "C" fn robot_frame(fighter: &mut L2CFighterCommon) {
    // To make this only work for certain slots, check Mod Workshop episode 24.
    let eff_handle = WorkModule::get_int(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_INT_LAMP_EFEFCT_ID) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        EffectModule::kill(fighter.module_accessor, eff_handle, false, false);
    }
}

pub fn install() {
    Agent::new("robot")
        .on_line(Main, robot_frame)
        .install();
}
