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

unsafe extern "C" fn trail_effect_fsmash(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3_fire"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}
else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 14.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
                EffectModule::enable_sync_init_pos_last(agent.module_accessor);
                macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3_ice"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
            }
            frame(agent.lua_state_agent, 25.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
                macros::AFTER_IMAGE_OFF(agent, 4);
            }
        }
    }

    unsafe extern "C" fn trail_effect_fsmashup(agent: &mut L2CAgentBase) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3_fire"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        }
        frame(agent.lua_state_agent, 23.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
            macros::AFTER_IMAGE_OFF(agent, 4);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 14.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
                EffectModule::enable_sync_init_pos_last(agent.module_accessor);
                macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
            }
            frame(agent.lua_state_agent, 25.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
                macros::AFTER_IMAGE_OFF(agent, 4);
            }
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
                if macros::is_excute(agent) {
                    macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                }
                frame(agent.lua_state_agent, 14.0);
                if macros::is_excute(agent) {
                    macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                    macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
                    EffectModule::enable_sync_init_pos_last(agent.module_accessor);
                    macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3_ice"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                }
                frame(agent.lua_state_agent, 23.0);
                if macros::is_excute(agent) {
                    macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
                }
                frame(agent.lua_state_agent, 25.0);
                if macros::is_excute(agent) {
                    macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
                    macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
                    macros::AFTER_IMAGE_OFF(agent, 4);
                }
            }
        }
        unsafe extern "C" fn trail_effect_fsmashdown(agent: &mut L2CAgentBase) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 14.0);
            if macros::is_excute(agent) {
                macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
                EffectModule::enable_sync_init_pos_last(agent.module_accessor);
                macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3_fire"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
            }
            frame(agent.lua_state_agent, 23.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
            }
            frame(agent.lua_state_agent, 25.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
                macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_fire"), false, true);
                macros::AFTER_IMAGE_OFF(agent, 4);
            }
        }
        else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
                if macros::is_excute(agent) {
                    macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                }
                frame(agent.lua_state_agent, 14.0);
                if macros::is_excute(agent) {
                    macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                    macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
                    EffectModule::enable_sync_init_pos_last(agent.module_accessor);
                    macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                }
                frame(agent.lua_state_agent, 23.0);
                if macros::is_excute(agent) {
                    macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
                }
                frame(agent.lua_state_agent, 25.0);
                if macros::is_excute(agent) {
                    macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
                    macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light"), false, true);
                    macros::AFTER_IMAGE_OFF(agent, 4);
                }
            }
            else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
                    if macros::is_excute(agent) {
                        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    }
                    frame(agent.lua_state_agent, 14.0);
                    if macros::is_excute(agent) {
                        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                        macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(agent, Hash40::new("trail_keyblade_light_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.25, true);
                        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
                        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3_ice"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0, 2, 0, Hash40::new("haver"), 0, 13.8, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
                    }
                    frame(agent.lua_state_agent, 23.0);
                    if macros::is_excute(agent) {
                        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
                    }
                    frame(agent.lua_state_agent, 25.0);
                    if macros::is_excute(agent) {
                        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
                        macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_light_ice"), false, true);
                        macros::AFTER_IMAGE_OFF(agent, 4);
                    }
                }
            }
    unsafe extern "C" fn trail_effect_dsmash(agent: &mut L2CAgentBase) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_lw_flash_fire"), Hash40::new("haver"), 0, 11, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_lw_speedline"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("trail_smash_lw_impact_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("trail_keyblade_flare_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
            macros::EFFECT(agent, Hash40::new("trail_smash_lw_attack_fire"), Hash40::new("haver"), 0, -2, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_fire"), false, true);
        }
        frame(agent.lua_state_agent, 57.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_lw_flash"), Hash40::new("haver"), 0, 11, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_lw_speedline"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("trail_smash_lw_impact"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
            macros::EFFECT(agent, Hash40::new("trail_smash_lw_attack"), Hash40::new("haver"), 0, -2, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare"), false, true);
        }
        frame(agent.lua_state_agent, 57.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_lw_flash_ice"), Hash40::new("haver"), 0, 11, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_lw_speedline"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("trail_smash_lw_impact_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("trail_keyblade_flare_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.4);
            macros::EFFECT(agent, Hash40::new("trail_smash_lw_attack_ice"), Hash40::new("haver"), 0, -2, 0, 90, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("trail_keyblade_flare_ice"), false, true);
        }
        frame(agent.lua_state_agent, 57.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), -0.9, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
}


unsafe extern "C" fn trail_effect_usmash(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 0{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_keyblade_fire"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_speedline_fire"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("trail_smash_hi_flash2_fire"), Hash40::new("haver"), 0, 10, -0.5, 0, 0, 0, 1.2, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_flash_fire"), Hash40::new("haver"), 0, 10, -0.5, 0, 0, 0, 1.2, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash_fire"), -1);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash2_fire"), -1);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 2{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_keyblade"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_speedline"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("trail_smash_hi_flash2"), Hash40::new("haver"), 0, 10, -0.5, 0, 0, 0, 1.2, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_flash"), Hash40::new("haver"), 0, 10, -0.5, 0, 0, 0, 1.2, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash"), -1);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash2"), -1);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND) == 1{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_keyblade_ice"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_speedline_ice"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("trail_smash_hi_flash2_ice"), Hash40::new("haver"), 0, 10, -0.5, 0, 0, 0, 1.2, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::EFFECT_FOLLOW(agent, Hash40::new("trail_smash_hi_flash_ice"), Hash40::new("haver"), 0, 10, -0.5, 0, 0, 0, 1.2, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 25.0);
        if macros::is_excute(agent) {
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash_ice"), -1);
            macros::EFFECT_DETACH_KIND(agent, Hash40::new("trail_smash_hi_flash2_ice"), -1);
        }
    }
}

    pub fn install(){
    Agent::new("trail")
        .effect_acmd("effect_attacks4", trail_effect_fsmash, Priority::Default)
        .effect_acmd("effect_attacks4hi", trail_effect_fsmashup, Priority::Default)
        .effect_acmd("effect_attacks4lw", trail_effect_fsmashdown, Priority::Default)
        .effect_acmd("effect_attackhi4", trail_effect_usmash, Priority::Default)
        .effect_acmd("effect_attacklw4", trail_effect_dsmash, Priority::Default)






    .install();
}