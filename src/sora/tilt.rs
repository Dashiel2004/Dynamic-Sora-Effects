use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*, std::hash,

};


unsafe extern "C" fn trail_effect_utilt(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_hi_fire"), Hash40::new("top"), 0, 22, 0, 0, -90, -70, 1.05, true);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_fire"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_hi"), Hash40::new("top"), 0, 22, 0, 0, -90, -70, 1.05, true);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_hi_ice"), Hash40::new("top"), 0, 22, 0, 0, -90, -70, 1.05, true);
        }
        frame(agent.lua_state_agent, 33.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_ice"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
        frame(agent.lua_state_agent, 50.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
unsafe extern "C" fn trail_effect_dtilt(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_fire"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_ice"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
}

unsafe extern "C" fn trail_effect_ftilt1(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_s3s_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_s3s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        frame(agent.lua_state_agent, 12.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_s3s_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
        }
    }
}

unsafe extern "C" fn trail_effect_ftilt2(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("trail_keyblade_speedline_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("trail_keyblade_speedline"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FLW_POS(agent, Hash40::new("trail_keyblade_speedline_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
        }
    }
}

unsafe extern "C" fn trail_effect_ftilt3(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_s33_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_s33"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_atk_slash_s33_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
        }
    }
}

pub fn install(){
    Agent::new("trail")
        .effect_acmd("effect_attackhi3", trail_effect_utilt, Priority::Default)
        .effect_acmd("effect_attacklw3", trail_effect_dtilt, Priority::Default)
        .effect_acmd("effect_attacks3", trail_effect_ftilt1, Priority::Default)
        .effect_acmd("effect_attacks32", trail_effect_ftilt2, Priority::Default)
        .effect_acmd("effect_attacks33", trail_effect_ftilt3, Priority::Default)
    .install();
}