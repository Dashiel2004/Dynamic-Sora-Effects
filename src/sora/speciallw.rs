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


unsafe extern "C" fn trail_effect_dspecialair(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 13, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_RATE(agent, 0.7);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_fire"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_counter_attack_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE_OFF(agent, 3);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_fire"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0.5, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_fire"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1.25, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 0);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack_fire"), -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 13, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_RATE(agent, 0.7);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE_OFF(agent, 3);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0.5, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1.25, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 0);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack"), -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 13, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::LAST_EFFECT_SET_RATE(agent, 0.7);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_ice"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_counter_attack_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::AFTER_IMAGE_OFF(agent, 3);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_ice"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0.5, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_ice"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1.25, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 0);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack_ice"), -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
        }
    }
}


unsafe extern "C" fn trail_effect_dspecial(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 13, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.7);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_fire"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_counter_attack_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::AFTER_IMAGE_OFF(agent, 3);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_fire"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0.5, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_fire"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1.25, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 0);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack_fire"), -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 13, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.7);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_counter_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::AFTER_IMAGE_OFF(agent, 3);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0.5, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1.25, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 0);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack"), -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 13, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 0.7);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_ice"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_counter_attack_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::AFTER_IMAGE_OFF(agent, 3);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1_ice"), Hash40::new("tex_trail_keyblade2"), 14, Hash40::new("haver"), 0, 2, 0.5, Hash40::new("haver"), 0, 19, 1, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_ice"), Hash40::new("haver"), 0, 5, 0, 0, 0, 0, 1.25, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::AFTER_IMAGE_OFF(agent, 0);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_counter_attack_ice"), -1);
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
        }
    }
}



pub fn install(){
    Agent::new("trail")
        .effect_acmd("effect_specialairlw", trail_effect_dspecialair, Priority::Default)
        .effect_acmd("effect_speciallw", trail_effect_dspecial, Priority::Default)
    .install();
}