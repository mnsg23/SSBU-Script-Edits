use {
	smash::{
		app::{
			AttackHeight,
			HitStatus,
			lua_bind::*,
			sv_animcmd
		},
		hash40,
		lib::lua_const::*,
		lua2cpp::L2CAgentBase,
		phx::*
	},
	smash_script::*,
	smashline::*
};

unsafe extern "C" fn kirby_effect_kroolspecialairnspitb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn kirby_effect_kroolspecialairnspitf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn kirby_effect_kroolspecialnspitb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn kirby_effect_kroolspecialnspitf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn kirby_sound_kroolspecialnspitb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n10"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) == true {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n05"));
		}
	} else {
		sv_animcmd::frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n06"));
		}
	}
}

unsafe extern "C" fn kirby_sound_kroolspecialnspitf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) == true {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n05"));
		}
	} else {
		sv_animcmd::frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n06"));
		}
	}
}

unsafe extern "C" fn kirby_sound_kroolspecialnspithi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n10"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) == true {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n05"));
		}
	} else {
		sv_animcmd::frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n06"));
		}
	}
}

unsafe extern "C" fn krool_effect_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 35, 3, 90, 0, 0, 1.7, 0, 0, 0, 0, 0, 0, true, 0.4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FLIP(fighter, Hash40::new("krool_smash_lw_ground"), Hash40::new("krool_smash_lw_ground"), Hash40::new("top"), 0, 0, 6, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
		macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 6, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
	}
}

unsafe extern "C" fn krool_effect_attacks3hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), -4, 16.5, 5, -10, -55, 16, 1.4, true, *EF_FLIP_YZ, 0.4);
		macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("krool_attack_s3_arc"), Hash40::new("krool_attack_s3_arc"), Hash40::new("top"), 4, 18, 5, -351, -55, 185, 1.4, true, *EF_FLIP_YZ, 0.4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_ALPHA(fighter, Hash40::new("krool_attack_s3_impact"), Hash40::new("top"), 0, 20, 20, 0, 0, 0, 1.7, 0, 0, 0, 0, 0, 360, true, 1);
	}
}

unsafe extern "C" fn krool_effect_specialairnspitb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn krool_effect_specialairnspitf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn krool_effect_specialairsthrow(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("krool_crownthrow"), Hash40::new("top"), 0, 11, 26, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
}

unsafe extern "C" fn krool_effect_specialnspitb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn krool_effect_specialnspitf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("krool_cannon_shot"), Hash40::new("haver"), 0, 2, 25, 0, 0, 0, 1.3, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_DETACH_KIND(fighter, Hash40::new("krool_cannon_shot"), -1);
	}
}

unsafe extern "C" fn krool_expression_attacklw4(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	sv_animcmd::execute(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
	}
}

unsafe extern "C" fn krool_game_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 4.0, 7.0, 8.5, 8.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 15, 0, 30, 2.5, 0.0, 9.0, 12.5, Some(0.0), Some(9.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 10, 0, 30, 2.5, 0.0, 9.0, 18.5, Some(0.0), Some(9.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 2.5, 0.0, 9.0, 18.5, Some(0.0), Some(9.0), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
	}
}

unsafe extern "C" fn krool_game_attack12(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 5.0, 8.0, 8.0, 8.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 15, 0, 30, 3.0, 0.0, 8.5, 16.5, Some(0.0), Some(8.5), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 10, 0, 30, 3.0, 0.0, 8.5, 21.0, Some(0.0), Some(8.5), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 30, 3.0, 0.0, 8.5, 21.0, Some(0.0), Some(8.5), Some(10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
}

unsafe extern "C" fn krool_game_attack13(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 2.0, 7.0, 10.0, 10.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 105, 0, 50, 5.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 19.0, 361, 75, 0, 60, 7.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 19.0, 270, 100, 0, 20, 7.5, 7.7, 3.5, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 19.0, 361, 75, 0, 60, 7.5, 7.7, 3.5, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn krool_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 76, 0, 60, 6.5, 0.0, 8.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 361, 76, 0, 60, 6.75, 0.0, 7.35, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 16.0, 361, 76, 0, 60, 7.0, 0.0, 6.2, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 11.5, 17.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 11.0, 14.75);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 10.5, 12.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 76, 0, 60, 5.0, 0.0, 15.0, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 76, 0, 60, 5.5, 0.0, 13.0, 9.75, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 361, 76, 0, 60, 6.0, 0.0, 11.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn krool_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		macros::SET_SPEED_EX(fighter, 0, 2.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 90, 57, 0, 90, 8.75, 0.0, 22.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 90, 57, 0, 90, 6.5, 0.0, 22.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 70.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 95.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

unsafe extern "C" fn krool_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 270, 100, 0, 20, 7.0, 0.0, -1.5, 0.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 270, 100, 0, 20, 8.5, 0.0, -6.5, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 6.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 7.0, 0.0, -6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn krool_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 2.5);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 85, 0, 40, 10.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 85, 0, 40, 9.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 31.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn krool_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 15.0, 45, 60, 0, 100, 8.0, 4.0, 3.5, 3.5, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 45, 65, 0, 90, 6.0, 4.0, 3.5, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
		JostleModule::set_status(fighter.module_accessor, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
	}
}

unsafe extern "C" fn krool_game_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 2.5);
		macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 13.0, 50, 86, 0, 60, 4.5, 2.2, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 13.0, 50, 86, 0, 60, 5.0, 7.5, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 9.0, 85, 85, 0, 60, 4.5, 2.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 9.0, 85, 85, 0, 60, 5.0, 7.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 17.0, 85, 80, 0, 60, 8.0, 7.5, 1.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 14.0, 361, 80, 0, 60, 8.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 14.0, 361, 80, 0, 60, 5.0, 7.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 75, 0, 70, 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(14.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		AttackModule::clear(fighter.module_accessor, 1, false);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 2.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 13.0, 361, 80, 0, 60, 5.0, 2.5, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 3.5, 16.5);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 40, 65, 0, 20, 5.0, 0.0, 3.5, 16.5, Some(0.0), Some(3.5), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 361, 80, 0, 60, 5.0, 0.0, 3.5, 16.5, Some(0.0), Some(3.5), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 85, 60, 0, 100, 10.0, 0.0, 5.5, 13.0, Some(0.0), Some(5.5), Some(23.0), 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	macros::FT_MOTION_RATE(fighter, 1.2);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn krool_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 4.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	sv_animcmd::execute(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 0.4);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 40, 76, 0, 70, 6.0, 0.0, 15.5, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 40, 76, 0, 70, 6.0, 0.0, 15.5, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 20.0, 40, 76, 0, 70, 6.0, 0.0, 15.5, 10.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 5.0, -4.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 5.0, 3.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 5.0, 10.5);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 4.0, -4.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 4.0, 3.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 4.0, 10.5);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 60, 0, 100, 11.0, 0.0, 9.0, 23.5, Some(0.0), Some(9.0), Some(-13.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 2.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 70, 0, 80, 5.0, 0.0, 11.0, 11.0, Some(0.0), Some(11.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("havel"), 13.0, 361, 70, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.2);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn krool_game_attacks3hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 2.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 70, 0, 80, 5.0, 0.0, 11.0, 11.0, Some(0.0), Some(11.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("havel"), 13.0, 361, 70, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.2);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn krool_game_attacks3lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 2.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 70, 0, 80, 5.0, 0.0, 11.0, 11.0, Some(0.0), Some(11.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("havel"), 13.0, 361, 70, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.2);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn krool_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.4);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	macros::FT_MOTION_RATE(fighter, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 22.0, 361, 93, 0, 40, 6.0, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 22.0, 361, 93, 0, 40, 4.5, -2.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_attacks4hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.4);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	macros::FT_MOTION_RATE(fighter, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 22.0, 361, 93, 0, 40, 6.0, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 22.0, 361, 93, 0, 40, 4.5, -2.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_attacks4lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.4);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	macros::FT_MOTION_RATE(fighter, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 22.0, 361, 93, 0, 40, 6.0, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 22.0, 361, 93, 0, 40, 4.5, -2.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_catchattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 361, 100, 30, 0, 8.0, 0.0, 10.0, 11.0, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_game_finaldash(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		macros::CAM_ZOOM_OUT(fighter);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 0, 1, 160, 14.0, 0.0, 12.0, 3.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
	}
}

unsafe extern "C" fn krool_game_finalend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_KROOL_FINAL, 0, 10.0, 60, 145, 0, 80, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_FINAL_ABS_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 36.0);
	macros::FT_MOTION_RATE(fighter, 0.9);
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	macros::FT_MOTION_RATE(fighter, 0.7);
}

unsafe extern "C" fn krool_game_finalend_com(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_KROOL_FINAL, 0, 10.0, 60, 145, 0, 80, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_FINAL_ABS_SET);
	}
}

unsafe extern "C" fn krool_game_finalstart(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		macros::SLOW_OPPONENT(fighter, 30.0, 60.0);
		macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 44, 0, 0, 0);
	}
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) == false {
		sv_animcmd::frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::FT_SET_FINAL_FEAR_FACE(fighter, 60);
			macros::REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
			macros::FT_START_CUTIN(fighter);
		}
	} else {
		if macros::is_excute(fighter) {
			camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 0, 4, 6, 0);
			macros::CAM_ZOOM_IN_arg5(fighter, 4.0, 0.0, 1.8, -0.1, 0.0);
			camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
			macros::FT_START_CUTIN(fighter);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			macros::CAM_ZOOM_OUT(fighter);
		}
	}
	sv_animcmd::frame(fighter.lua_state_agent, 31.0);
	if macros::is_excute(fighter) {
		if PostureModule::scale(fighter.module_accessor) < 1.0 {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 368, 100, 0, 0, 8.0, 0.0, 6.0, 13.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
			AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 60.0, y: 4.0}, 1.0 as u32, false);
		} else {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 20, 0, 1, 85, 8.0, 0.0, 6.0, 13.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		}
		AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 41.0);
	if macros::is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
	}
}

unsafe extern "C" fn krool_game_specialhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 120);
	}
}

unsafe extern "C" fn krool_game_specialhiairend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn krool_game_speciallwhit(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ATTACK_DAMAGE_REQUEST);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_TURN);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 45, 60, 0, 100, 14.0, 0.0, 13.0, 18.0, Some(0.0), Some(13.0), Some(12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_WAIST_DAMAGE_REQUEST);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	macros::FT_MOTION_RATE(fighter, 0.7);
	sv_animcmd::frame(fighter.lua_state_agent, 48.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn krool_game_speciallwturn(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 45, 60, 0, 100, 14.0, 0.0, 13.0, -18.0, Some(0.0), Some(13.0), Some(-12.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_WAIST_DAMAGE_REQUEST);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	macros::FT_MOTION_RATE(fighter, 0.7);
	sv_animcmd::frame(fighter.lua_state_agent, 53.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn krool_game_specialnend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn krool_game_specialnfire(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.25);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 51.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 70.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
	}
}

unsafe extern "C" fn krool_game_specialnloop(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
		macros::CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(18.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 9.5, 10.7, Some(0.0), Some(9.5), Some(20.5), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_A);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 160, 100, 30, 0, 9.0, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(27.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(28.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(34.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(38.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
	}
}

unsafe extern "C" fn krool_game_specialnspitb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.25);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 40, 47, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 41.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn krool_game_specialnspitbattackabs(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 40, 47, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
}

unsafe extern "C" fn krool_game_specialnspitf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.25);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 40, 50, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT_TYPE_DECIDE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn krool_game_specialnspithi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.25);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 90, 48, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 36.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn krool_game_specialnsuction(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 12);
	}
}

unsafe extern "C" fn krool_game_specialnswallow(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SWALLOW_COMP);
		MotionModule::set_rate(fighter.module_accessor, 0.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
}

unsafe extern "C" fn krool_game_specialsthrow(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_S_FLAG_ENABLE_SUPER_ARMOR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 0.0, 361, 100, 30, 0, 5.0, 5.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	macros::FT_MOTION_RATE(fighter, 1.2);
	if macros::is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_CROWN, false, 0);
		VisibilityModule::set_int64(fighter.module_accessor, hash40("crown") as i64, hash40("crown_hide") as i64);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 36.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 64.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn krool_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 35, 55, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 31.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
		macros::CHECK_FINISH_CAMERA(fighter, 31, 2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: -1.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn krool_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 60, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 75, 0, 70, 1.0, 4.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 75, 0, 70, 6.0, 0.0, 3.0, 21.0, Some(0.0), Some(3.0), Some(25.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		AttackModule::set_size(fighter.module_accessor, 0, 6.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::CHECK_FINISH_CAMERA(fighter, 24, 0);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 12.0, y: -2.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn krool_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 16.0, 90, 38, 0, 110, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 47.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_STOP);
		macros::CHECK_FINISH_CAMERA(fighter, 3, 10);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 48.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 55.0);
	if macros::is_excute(fighter) {
		macros::SA_SET(fighter, *SITUATION_KIND_GROUND);
		macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND);
	}
}

unsafe extern "C" fn krool_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 50, 70, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_bury_r"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_bury_r"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_THROW_LW_FLAG_BURY);
	}
}

unsafe extern "C" fn krool_game_visualscene04(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_KROOL_FINAL, 0, 40.0, 361, 0, 1, 120, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		SlowModule::set_whole(fighter.module_accessor, 4 as u8, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		SlowModule::clear_whole(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_WORK_ID_FLAG_FINAL_VISUAL_SCENE_04_DAMAGE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		SlowModule::set_whole(fighter.module_accessor, 2 as u8, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 55.0);
	if macros::is_excute(fighter) {
		SlowModule::clear_whole(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_sound_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_attackdash_01"));
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_krool_rnd_attack"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_krool_attackdash_01"));
		macros::PLAY_SE(fighter, Hash40::new("se_krool_attackdash_02"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_attackdash_03"));
	}
}

unsafe extern "C" fn krool_sound_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_krool_attack06"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_smash_s01"));
	}
}

unsafe extern "C" fn krool_sound_specialnspitb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n10"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) == true {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n05"));
		}
	} else {
		sv_animcmd::frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_INT_OPPONENT_REACTION_FRAME) < 40 {
				macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n11"));
			} else {
				macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n06"));
			}
		}
	}
}

unsafe extern "C" fn krool_sound_specialnspitf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) == true {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n05"));
		}
	} else {
		sv_animcmd::frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_INT_OPPONENT_REACTION_FRAME) < 40 {
				macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n11"));
			} else {
				macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n06"));
			}
		}
	}
}

unsafe extern "C" fn krool_sound_specialnspithi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n10"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) == true {
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n05"));
		}
	} else {
		sv_animcmd::frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_INT_OPPONENT_REACTION_FRAME) < 40 {
				macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n11"));
			} else {
				macros::PLAY_SE(fighter, Hash40::new("se_krool_special_n06"));
			}
		}
	}
}

unsafe extern "C" fn krool_backpack_game_fly(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("wingl1"), 3.0, 80, 30, 0, 90, 6.0, 2.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn krool_crown_game_shot(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 65, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -5, 0.0, 44, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn krool_crown_game_throw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 65, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, Some(-4.8), Some(-6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -5, 0.0, 44, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 65, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -5, 0.0, 44, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 39.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 8.0, 65, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, -4, 0.0, 44, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn krool_ironball_game_hop(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn krool_ironball_game_shoot(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 60, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_ironball_game_spitshoot(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 60, 85, 0, 60, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(fighter.module_accessor);
	}
}

unsafe extern "C" fn krool_spitball_game_fly(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 60, 85, 0, 70, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_KROOL_SPITBALL_INSTANCE_WORK_ID_FLAG_SHOW_OWNER);
	}
}

pub fn install() {
	Agent::new("kirby")
		.effect_acmd("effect_kroolspecialairnspitb", kirby_effect_kroolspecialairnspitb)
		.effect_acmd("effect_kroolspecialairnspitf", kirby_effect_kroolspecialairnspitf)
		.effect_acmd("effect_kroolspecialnspitb", kirby_effect_kroolspecialnspitb)
		.effect_acmd("effect_kroolspecialnspitf", kirby_effect_kroolspecialnspitf)
		.sound_acmd("sound_kroolspecialnspitb", kirby_sound_kroolspecialnspitb)
		.sound_acmd("sound_kroolspecialairnspitb", kirby_sound_kroolspecialnspitb)
		.sound_acmd("sound_kroolspecialnspitf", kirby_sound_kroolspecialnspitf)
		.sound_acmd("sound_kroolspecialairnspitf", kirby_sound_kroolspecialnspitf)
		.sound_acmd("sound_kroolspecialnspithi", kirby_sound_kroolspecialnspithi)
		.sound_acmd("sound_kroolspecialairnspithi", kirby_sound_kroolspecialnspithi)
		.install();
	Agent::new("krool")
		.effect_acmd("effect_attacklw4", krool_effect_attacklw4)
		.effect_acmd("effect_attacks3hi", krool_effect_attacks3hi)
		.effect_acmd("effect_specialairnspitb", krool_effect_specialairnspitb)
		.effect_acmd("effect_specialairnspitf", krool_effect_specialairnspitf)
		.effect_acmd("effect_specialairsthrow", krool_effect_specialairsthrow)
		.effect_acmd("effect_specialnspitb", krool_effect_specialnspitb)
		.effect_acmd("effect_specialnspitf", krool_effect_specialnspitf)
		.expression_acmd("expression_attacklw4", krool_expression_attacklw4)
		.game_acmd("game_attack11", krool_game_attack11)
		.game_acmd("game_attack12", krool_game_attack12)
		.game_acmd("game_attack13", krool_game_attack13)
		.game_acmd("game_attackairb", krool_game_attackairb)
		.game_acmd("game_attackairf", krool_game_attackairf)
		.game_acmd("game_attackairhi", krool_game_attackairhi)
		.game_acmd("game_attackairlw", krool_game_attackairlw)
		.game_acmd("game_attackairn", krool_game_attackairn)
		.game_acmd("game_attackdash", krool_game_attackdash)
		.game_acmd("game_attackhi3", krool_game_attackhi3)
		.game_acmd("game_attackhi4", krool_game_attackhi4)
		.game_acmd("game_attacklw3", krool_game_attacklw3)
		.game_acmd("game_attacklw4", krool_game_attacklw4)
		.game_acmd("game_attacks3", krool_game_attacks3)
		.game_acmd("game_attacks3hi", krool_game_attacks3hi)
		.game_acmd("game_attacks3lw", krool_game_attacks3lw)
		.game_acmd("game_attacks4", krool_game_attacks4)
		.game_acmd("game_attacks4hi", krool_game_attacks4hi)
		.game_acmd("game_attacks4lw", krool_game_attacks4lw)
		.game_acmd("game_catchattack", krool_game_catchattack)
		.game_acmd("game_finaldash", krool_game_finaldash)
		.game_acmd("game_finalend", krool_game_finalend)
		.game_acmd("game_finalairend", krool_game_finalend)
		.game_acmd("game_finalend_com", krool_game_finalend_com)
		.game_acmd("game_finalstart", krool_game_finalstart)
		.game_acmd("game_finalairstart", krool_game_finalstart)
		.game_acmd("game_specialhi", krool_game_specialhi)
		.game_acmd("game_specialhiairend", krool_game_specialhiairend)
		.game_acmd("game_speciallwhit", krool_game_speciallwhit)
		.game_acmd("game_specialairlwhit", krool_game_speciallwhit)
		.game_acmd("game_speciallwturn", krool_game_speciallwturn)
		.game_acmd("game_specialairlwturn", krool_game_speciallwturn)
		.game_acmd("game_specialnend", krool_game_specialnend)
		.game_acmd("game_specialairnend", krool_game_specialnend)
		.game_acmd("game_specialnfire", krool_game_specialnfire)
		.game_acmd("game_specialairnfire", krool_game_specialnfire)
		.game_acmd("game_specialnloop", krool_game_specialnloop)
		.game_acmd("game_specialairnloop", krool_game_specialnloop)
		.game_acmd("game_specialnspitb", krool_game_specialnspitb)
		.game_acmd("game_specialairnspitb", krool_game_specialnspitb)
		.game_acmd("game_specialnspitbattackabs", krool_game_specialnspitbattackabs)
		.game_acmd("game_specialnspitf", krool_game_specialnspitf)
		.game_acmd("game_specialairnspitf", krool_game_specialnspitf)
		.game_acmd("game_specialnspithi", krool_game_specialnspithi)
		.game_acmd("game_specialairnspithi", krool_game_specialnspithi)
		.game_acmd("game_specialnsuction", krool_game_specialnsuction)
		.game_acmd("game_specialairnsuction", krool_game_specialnsuction)
		.game_acmd("game_specialnswallow", krool_game_specialnswallow)
		.game_acmd("game_specialairnswallow", krool_game_specialnswallow)
		.game_acmd("game_specialsthrow", krool_game_specialsthrow)
		.game_acmd("game_specialairsthrow", krool_game_specialsthrow)
		.game_acmd("game_throwb", krool_game_throwb)
		.game_acmd("game_throwf", krool_game_throwf)
		.game_acmd("game_throwhi", krool_game_throwhi)
		.game_acmd("game_throwlw", krool_game_throwlw)
		.game_acmd("game_visualscene04", krool_game_visualscene04)
		.sound_acmd("sound_attackdash", krool_sound_attackdash)
		.sound_acmd("sound_attacks4", krool_sound_attacks4)
		.sound_acmd("sound_attacks4hi", krool_sound_attacks4)
		.sound_acmd("sound_attacks4lw", krool_sound_attacks4)
		.sound_acmd("sound_specialnspitb", krool_sound_specialnspitb)
		.sound_acmd("sound_specialairnspitb", krool_sound_specialnspitb)
		.sound_acmd("sound_specialnspitf", krool_sound_specialnspitf)
		.sound_acmd("sound_specialairnspitf", krool_sound_specialnspitf)
		.sound_acmd("sound_specialnspithi", krool_sound_specialnspithi)
		.sound_acmd("sound_specialairnspithi", krool_sound_specialnspithi)
		.install();
	Agent::new("krool_backpack")
		.game_acmd("game_fly", krool_backpack_game_fly)
		.game_acmd("game_flywind", krool_backpack_game_fly)
		.install();
	Agent::new("krool_crown")
		.game_acmd("game_shot", krool_crown_game_shot)
		.game_acmd("game_throw", krool_crown_game_throw)
		.install();
	Agent::new("krool_ironball")
		.game_acmd("game_hop", krool_ironball_game_hop)
		.game_acmd("game_spithop", krool_ironball_game_hop)
		.game_acmd("game_shoot", krool_ironball_game_shoot)
		.game_acmd("game_spitshoot", krool_ironball_game_spitshoot)
		.install();
	Agent::new("krool_spitball")
		.game_acmd("game_fly", krool_spitball_game_fly)
		.install();
}
