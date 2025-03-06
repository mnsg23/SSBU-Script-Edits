use {
	smash::{
		app::{
			AttackHeight,
			HitStatus,
			lua_bind::*,
			sv_animcmd
		},
		lib::lua_const::*,
		lua2cpp::L2CAgentBase,
		phx::*
	},
	smash_script::*,
	smashline::*
};

unsafe extern "C" fn purin_effect_appeallwl(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("purin_appeal_lw"), Hash40::new("hip"), 0, 0, 0, -90, -90, 0, 1, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 90.0);
	if macros::is_excute(fighter) {
		macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn purin_effect_throwlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 60.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("trans"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 65.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 19, 4, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
	}
}

unsafe extern "C" fn purin_expression_attack11(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
	}
}

unsafe extern "C" fn purin_expression_attack12(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
	}
}

unsafe extern "C" fn purin_expression_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

unsafe extern "C" fn purin_expression_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

unsafe extern "C" fn purin_expression_attacklw3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 12);
	}
}

unsafe extern "C" fn purin_expression_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
		sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 8);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 8);
	}
	for _ in 0..2 {
		if macros::is_excute(fighter) {
			ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 10.0);
	}
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}		
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 65.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
	}
}

unsafe extern "C" fn purin_game_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.5, 0.0, 5.8, 7.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 5, 0, 25, 2.5, 0.0, 5.8, 10.5, Some(0.0), Some(5.8), Some(7.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 5, 0, 25, 2.5, 0.0, 5.8, 10.5, Some(0.0), Some(5.8), Some(7.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
	}
}

unsafe extern "C" fn purin_game_attack12(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.3333333);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 120, 0, 40, 4.0, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 40, 110, 0, 20, 4.5, 0.0, 4.0, -12.5, Some(0.0), Some(4.0), Some(-8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn purin_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 105, 0, 20, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 105, 0, 20, 3.5, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 115, 0, 20, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 115, 0, 20, 3.5, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("top"), 0.0, 4.0, 8.7);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 125, 0, 20, 4.5, 0.0, 4.0, 8.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 125, 0, 20, 3.5, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn purin_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 9.0, 85, 95, 0, 40, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 9.0, 85, 95, 0, 40, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 9.0, 85, 95, 0, 40, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn purin_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	for _ in 0..8 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 3.6, 0.0, -2.3, 1.0, Some(0.0), Some(-1.3), Some(3.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 366, 100, 80, 0, 3.6, 0.0, -2.3, 1.0, Some(0.0), Some(-1.3), Some(3.5), 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 366, 100, 80, 0, 3.0, 0.0, 3.5, 1.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 45, 180, 0, 30, 5.0, 0.0, -2.4, 3.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 45, 180, 0, 30, 3.0, 0.0, 3.5, 1.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn purin_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 3.7, 0.0, 3.0, 2.0, Some(0.0), Some(3.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 6.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 105, 0, 30, 3.5, 0.0, 3.0, 1.8, Some(0.0), Some(3.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::clear(fighter.module_accessor, 1, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 125, 0, 20, 3.5, 0.0, 3.0, 1.8, Some(0.0), Some(3.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 31.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn purin_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 12.0, 45, 70, 0, 80, 5.5, 0.0, 2.3, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 8.0, 45, 75, 0, 70, 4.5, 0.0, 2.3, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
		macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 9.0, 95, 115, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 9.0, 95, 115, 0, 50, 5.5, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 95, 115, 0, 50, 3.0, 0.0, 3.0, -6.5, Some(0.0), Some(3.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 2, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 90, 120, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 8.0, 90, 120, 0, 40, 5.5, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 16.0, 85, 100, 0, 30, 6.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 6.0, 361, 100, 0, 20, 3.0, 0.0, -8.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 6.0, 361, 100, 0, 20, 3.0, 0.0, -4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 6.0, 361, 100, 0, 20, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	macros::FT_MOTION_RATE(fighter, 0.6);
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn purin_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
		macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 14.0, 30, 97, 0, 30, 4.8, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("toel"), 14.0, 30, 97, 0, 30, 4.8, 0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 14.0, 30, 97, 0, 30, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("footl"), 14.0, 30, 97, 0, 30, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attacks3hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attacks3lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 361, 100, 0, 20, 3.5, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 16.0, 361, 110, 0, 20, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 16.0, 361, 110, 0, 20, 5.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 14.0, 361, 90, 0, 20, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 14.0, 361, 90, 0, 20, 4.0, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_catch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.5);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(8.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.75, 0.0, 5.2, 2.25, Some(0.0), Some(5.2), Some(10.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn purin_game_catchattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 30, 0, 6.5, 0.0, 7.0, 13.5, Some(0.0), Some(7.0), Some(9.5), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn purin_game_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.5);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 2.8, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(10.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.4, 0.0, 5.2, 2.6, Some(0.0), Some(5.2), Some(11.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn purin_game_catchturn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.5);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, -4.0, Some(0.0), Some(5.2), Some(-14.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.75, 0.0, 5.2, -2.25, Some(0.0), Some(5.2), Some(-16.25), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn purin_game_cliffattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 4.5, 0.0, 4.0, 9.5, Some(0.0), Some(4.0), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_final(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 15.0, 120, 0, 1, 75, 4.5, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(1.8), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 21.0, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_invalid_xlu(fighter.module_accessor, 0, true);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.64);
	sv_animcmd::frame(fighter.lua_state_agent, 67.0);
	macros::FT_MOTION_RATE(fighter, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 35.0, 50, 55, 0, 80, 7.0, 0.0, 1.6, -2.0, Some(0.0), Some(1.6), Some(2.3), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_invalid_xlu(fighter.module_accessor, 0, true);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_finalair(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 15.0, 120, 0, 1, 75, 4.5, 0.0, 0.0, -1.6, Some(0.0), Some(0.0), Some(1.8), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 21.0, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_invalid_xlu(fighter.module_accessor, 0, true);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter, 0.64);
	sv_animcmd::frame(fighter.lua_state_agent, 67.0);
	macros::FT_MOTION_RATE(fighter, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 35.0, 50, 55, 0, 80, 7.0, 0.0, 0.5, -2.2, Some(0.0), Some(0.5), Some(2.3), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_invalid_xlu(fighter.module_accessor, 0, true);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_specialairs(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 75, 75, 0, 50, 4.5, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_INPUT);
		WorkModule::set_int(fighter.module_accessor, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_BRAKE, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 31.0);
	if macros::is_excute(fighter) {
		WorkModule::set_int(fighter.module_accessor, *FIGHTER_PURIN_SPECIAL_S_ENERGY_MODE_FALL, *FIGHTER_PURIN_STATUS_SPECIAL_S_WORK_INT_ENERGY_MODE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_S_FLAG_CHANGE_ENERGY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 41.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

unsafe extern "C" fn purin_game_specialhil(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	purin_game_specialhisub(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 61.0);
	purin_game_specialhisub(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 100.0);
	purin_game_specialhisub(fighter);
	if macros::is_excute(fighter) {
		AttackModule::set_size(fighter.module_accessor, 0, 16.0);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_specialhisub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 50, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 11.5, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
}

unsafe extern "C" fn purin_game_speciallwl(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 25.0, 90, 55, 0, 110, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		JostleModule::set_status(fighter.module_accessor, true);
	}
}

unsafe extern "C" fn purin_game_specialn(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, false);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 60, 0, 70, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
	}
}

unsafe extern "C" fn purin_game_specials(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 75, 75, 0, 50, 4.5, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn purin_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 135, 35, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_STOP);
		macros::CHECK_FINISH_CAMERA(fighter, -15, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn purin_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 35, 48, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 5.0, 35, 85, 0, 70, 10.75, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::CHECK_FINISH_CAMERA(fighter, 13, 2);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.4);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 2.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn purin_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 36, 0, 130, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, -4, 9);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 2.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn purin_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 80, 120, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	for _ in 0..2 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 40, 100, 30, 0, 6.0, 0.0, 4.0, 3.2, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 40, 100, 30, 0, 6.0, 0.0, 4.0, 3.2, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 8.0);
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 40, 100, 30, 0, 6.0, 0.0, 4.0, 3.2, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::CHECK_FINISH_CAMERA(fighter, 3, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.1);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 61.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
}

unsafe extern "C" fn purin_sound_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_purin_rnd_attack01"));
		macros::PLAY_SE(fighter, Hash40::new("se_purin_swing_m"));
	}
}

unsafe extern "C" fn purin_sound_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_purin_rnd_attack01"));
		macros::PLAY_SE(fighter, Hash40::new("se_purin_swing_ll"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_purin_landing01"));
	}
}

unsafe extern "C" fn purin_sound_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_purin_rnd_attack_smash_s"));
		macros::PLAY_SE(fighter, Hash40::new("se_purin_swing_ll"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_purin_step_left_m"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_purin_step_right_m"));
	}
}

unsafe extern "C" fn purin_sound_throwf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_purin_rnd_attack01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_purin_throw_f01"));
	}
}

unsafe extern "C" fn purin_sound_throwlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_purin_rnd_attack01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_purin_throw_l01"));
	}
	for _ in 0..2 {
		sv_animcmd::wait(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_purin_throw_l01"));
		}
	}
}

pub fn install() {
	Agent::new("purin")
		.effect_acmd("effect_appeallwl", purin_effect_appeallwl, Priority::Default)
		.effect_acmd("effect_appeallwr", purin_effect_appeallwl, Priority::Default)
		.effect_acmd("effect_throwlw", purin_effect_throwlw, Priority::Default)
		.expression_acmd("expression_attack11", purin_expression_attack11, Priority::Default)
		.expression_acmd("expression_attack12", purin_expression_attack12, Priority::Default)
		.expression_acmd("expression_attackairf", purin_expression_attackairf, Priority::Default)
		.expression_acmd("expression_attackairn", purin_expression_attackairn, Priority::Default)
		.expression_acmd("expression_attacklw3", purin_expression_attacklw3, Priority::Default)
		.expression_acmd("expression_throwlw", purin_expression_throwlw, Priority::Default)
		.game_acmd("game_attack11", purin_game_attack11, Priority::Default)
		.game_acmd("game_attack12", purin_game_attack12, Priority::Default)
		.game_acmd("game_attackairb", purin_game_attackairb, Priority::Default)
		.game_acmd("game_attackairf", purin_game_attackairf, Priority::Default)
		.game_acmd("game_attackairhi", purin_game_attackairhi, Priority::Default)
		.game_acmd("game_attackairlw", purin_game_attackairlw, Priority::Default)
		.game_acmd("game_attackairn", purin_game_attackairn, Priority::Default)
		.game_acmd("game_attackdash", purin_game_attackdash, Priority::Default)
		.game_acmd("game_attackhi3", purin_game_attackhi3, Priority::Default)
		.game_acmd("game_attackhi4", purin_game_attackhi4, Priority::Default)
		.game_acmd("game_attacklw3", purin_game_attacklw3, Priority::Default)
		.game_acmd("game_attacklw4", purin_game_attacklw4, Priority::Default)
		.game_acmd("game_attacks3", purin_game_attacks3, Priority::Default)
		.game_acmd("game_attacks3hi", purin_game_attacks3hi, Priority::Default)
		.game_acmd("game_attacks3lw", purin_game_attacks3lw, Priority::Default)
		.game_acmd("game_attacks4", purin_game_attacks4, Priority::Default)
		.game_acmd("game_catch", purin_game_catch, Priority::Default)
		.game_acmd("game_catchattack", purin_game_catchattack, Priority::Default)
		.game_acmd("game_catchdash", purin_game_catchdash, Priority::Default)
		.game_acmd("game_catchturn", purin_game_catchturn, Priority::Default)
		.game_acmd("game_cliffattack", purin_game_cliffattack, Priority::Default)
		.game_acmd("game_final", purin_game_final, Priority::Default)
		.game_acmd("game_finalair", purin_game_finalair, Priority::Default)
		.game_acmd("game_specialairs", purin_game_specialairs, Priority::Default)
		.game_acmd("game_specialhil", purin_game_specialhil, Priority::Default)
		.game_acmd("game_specialairhil", purin_game_specialhil, Priority::Default)
		.game_acmd("game_specialhir", purin_game_specialhil, Priority::Default)
		.game_acmd("game_specialairhir", purin_game_specialhil, Priority::Default)
		.game_acmd("game_specialhisub", purin_game_specialhisub, Priority::Default)
		.game_acmd("game_speciallwl", purin_game_speciallwl, Priority::Default)
		.game_acmd("game_specialairlwl", purin_game_speciallwl, Priority::Default)
		.game_acmd("game_speciallwr", purin_game_speciallwl, Priority::Default)
		.game_acmd("game_specialairlwr", purin_game_speciallwl, Priority::Default)
		.game_acmd("game_specialn", purin_game_specialn, Priority::Default)
		.game_acmd("game_specialairn", purin_game_specialn, Priority::Default)
		.game_acmd("game_specials", purin_game_specials, Priority::Default)
		.game_acmd("game_throwb", purin_game_throwb, Priority::Default)
		.game_acmd("game_throwf", purin_game_throwf, Priority::Default)
		.game_acmd("game_throwhi", purin_game_throwhi, Priority::Default)
		.game_acmd("game_throwlw", purin_game_throwlw, Priority::Default)
		.sound_acmd("sound_attackairhi", purin_sound_attackairhi, Priority::Default)
		.sound_acmd("sound_attackdash", purin_sound_attackdash, Priority::Default)
		.sound_acmd("sound_attacks4", purin_sound_attacks4, Priority::Default)
		.sound_acmd("sound_throwf", purin_sound_throwf, Priority::Default)
		.sound_acmd("sound_throwlw", purin_sound_throwlw, Priority::Default)
		.install();
}
