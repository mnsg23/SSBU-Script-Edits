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

unsafe extern "C" fn wario_expression_attacklw4(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	sv_animcmd::execute(fighter.lua_state_agent, 3.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) == true {
		if macros::is_excute(fighter) {
			slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
		}
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 58.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
	}
}

unsafe extern "C" fn wario_expression_speciallwlr(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosionm"), 0);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::AREA_WIND_2ND_RAD(fighter, 1, 1.5, 0.02, 1000, 1, 0, 4, 32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		AreaModule::erase_wind(fighter.module_accessor, 1);
	}
}

unsafe extern "C" fn wario_game_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.2);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 15, 0, 40, 3.0, 0.0, 6.0, 9.0, Some(0.0), Some(6.5), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 180, 5, 0, 40, 3.0, 0.0, 4.0, 12.0, Some(0.0), Some(6.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 361, 5, 0, 40, 3.0, 0.0, 4.0, 12.0, Some(0.0), Some(6.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 1.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 1.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 1.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
	}
}

unsafe extern "C" fn wario_game_attack12(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 75, 105, 0, 60, 4.5, 0.0, 8.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 75, 105, 0, 60, 6.0, 3.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 14.0, 361, 92, 0, 40, 5.5, 6.0, 0.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 14.0, 361, 92, 0, 40, 3.5, 2.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn wario_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 94, 0, 40, 5.5, 0.0, 5.4, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 94, 0, 40, 3.5, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 98, 0, 40, 4.5, 0.0, 5.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 98, 0, 40, 3.0, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn wario_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 90, 90, 0, 40, 7.5, 0.0, 16.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn wario_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	for _ in 0..6 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 366, 100, 50, 0, 7.0, 0.0, -6.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 175, 0, 50, 7.0, 0.0, -6.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 71.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

unsafe extern "C" fn wario_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 40, 96, 0, 40, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 40, 96, 0, 40, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 40, 96, 0, 40, 3.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 10.0, 40, 96, 0, 40, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 4, 0, Hash40::new("armr"), 10.0, 40, 96, 0, 40, 3.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 4.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 40, 110, 0, 40, 4.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		AttackModule::clear(fighter.module_accessor, 1, false);
		AttackModule::clear(fighter.module_accessor, 2, false);
		AttackModule::clear(fighter.module_accessor, 3, false);
		AttackModule::clear(fighter.module_accessor, 4, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	macros::FT_MOTION_RATE(fighter, 0.75);
	sv_animcmd::frame(fighter.lua_state_agent, 47.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn wario_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 92, 0, 60, 4.0, 0.0, 5.0, 5.8, Some(0.0), Some(9.7), Some(5.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 45, 92, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 92, 0, 60, 4.0, 0.0, 8.0, 4.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 70, 92, 0, 60, 4.0, 0.0, 6.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 90, 77, 0, 70, 5.0, 0.0, 18.0, 5.0, Some(0.0), Some(18.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 90, 77, 0, 70, 5.0, 0.0, 12.0, 1.5, Some(0.0), Some(12.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 90, 77, 0, 70, 5.0, 0.0, 6.0, 1.5, Some(0.0), Some(6.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 90, 98, 0, 40, 4.0, 0.0, 14.5, 3.5, Some(0.0), Some(14.5), Some(-3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 90, 98, 0, 40, 4.0, 0.0, 10.25, 1.5, Some(0.0), Some(10.25), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 90, 98, 0, 40, 4.0, 0.0, 6.0, 1.5, Some(0.0), Some(6.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 17.0, 85, 92, 0, 40, 7.0, 4.5, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}

unsafe extern "C" fn wario_game_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 100, 0, 20, 2.0, 0.0, 1.8, 18.0, Some(0.0), Some(1.8), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	for _ in 0..12 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 180, 100, 30, 0, 8.0, 0.0, 6.0, -6.0, Some(0.0), Some(6.0), Some(6.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 30, 200, 0, 40, 8.0, 0.0, 6.0, -6.0, Some(0.0), Some(6.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 13.0, 361, 100, 0, 30, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 13.0, 361, 100, 0, 30, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attacks3hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 13.0, 361, 100, 0, 30, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 13.0, 361, 100, 0, 30, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attacks3lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 13.0, 361, 100, 0, 30, 5.7, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 13.0, 361, 100, 0, 30, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 20.0, 361, 95, 0, 30, 6.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 20.0, 361, 95, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_catch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(10.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.85, 0.0, 7.0, 2.15, Some(0.0), Some(7.0), Some(12.15), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn wario_game_catchattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 361, 100, 30, 0, 5.5, 0.0, 9.5, 7.5, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(12.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.5, 0.0, 7.0, 2.5, Some(0.0), Some(7.0), Some(13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn wario_game_cliffattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 11.0, Some(0.0), Some(5.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_finaldash(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 0, 1, 160, 8.0, 0.0, 7.7, -2.0, Some(0.0), Some(7.7), Some(5.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
	}
}

unsafe extern "C" fn wario_game_finaldashend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		macros::SET_SPEED_EX(fighter, 2, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_WARIO_FINAL, 0, 10.0, 60, 145, 0, 80, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_SET);
	}
}

unsafe extern "C" fn wario_game_finalend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_WARIO_FINAL, 0, 10.0, 60, 145, 0, 80, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_VISUAL_SCENE_FLAG_END_EXIT);
	}
}

unsafe extern "C" fn wario_game_finalvisualscene(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_WARIO_FINAL, 0, 1.5, 361, 0, 1, 20, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 72.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_1);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 77.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 82.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_2);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 88.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 96.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 105.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_3);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 113.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 121.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_1);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 130.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 137.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 144.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_2);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 153.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 162.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 171.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_3);
		WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 178.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 185.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 202.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_WARIO_FINAL, 0, 16.0, 361, 0, 1, 20, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, true);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ATTACK_ABS_RESET);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_ABS_HIT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_PRESET_4);
		WorkModule::set_float(fighter.module_accessor, 0.2, *FIGHTER_WARIO_STATUS_FINAL_WORK_FLOAT_TARGET_MOTION_RATE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_FINAL_FLAG_TARGET_MOTION_RATE_SET);
	}
}

unsafe extern "C" fn wario_game_landingairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 40, 140, 0, 60, 7.0, 0.0, 2.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_specialhijump(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 6.5, 0.0, 8.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 6.5, 0.0, 11.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 6.5, 0.0, 5.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 125, 100, 80, 0, 6.5, 0.0, 8.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 1.0, 125, 100, 80, 0, 6.5, 0.0, 11.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 1.0, 125, 100, 80, 0, 6.5, 0.0, 5.0, 0.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 170, 0, 60, 5.5, 0.0, 13.0, 7.5, Some(0.0), Some(13.0), Some(-7.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 6, 0, Hash40::new("top"), 4.0, 45, 170, 0, 60, 8.0, 0.0, 7.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE);
	}
}

unsafe extern "C" fn wario_game_speciallwflyr(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 35, 100, 0, 30, 13.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 80, 100, 0, 30, 7.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_speciallwlr(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 45, 80, 0, 60, 13.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_speciallwmr(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 100, 0, 30, 12.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_speciallwsr(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 20, 0, 9.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn wario_game_specialnbite(fighter: &mut L2CAgentBase) {
	loop {
		sv_animcmd::frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 2.5, 361, 100, 30, 0, 3.0, -0.5, 2.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BITE);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		sv_animcmd::wait_loop(fighter.lua_state_agent);
		sv_animcmd::frame_clear(fighter.lua_state_agent);
	}
}

unsafe extern "C" fn wario_game_specialnopenwait(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("head"), 3.7, -2.0, 2.5, 1.0, None, None, None, *FIGHTER_STATUS_KIND_BITTEN_WARIO_START, *COLLISION_SITUATION_MASK_A);
		macros::CATCH(fighter, 1, Hash40::new("head"), 5.4, -2.0, 2.0, 1.5, None, None, None, *FIGHTER_STATUS_KIND_BITTEN_WARIO_START, *COLLISION_SITUATION_MASK_G);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 45, 95, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
			macros::SEARCH(fighter, 0, 0, Hash40::new("head"), 4.5, -2.0, 5.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
		} else {
			macros::SEARCH(fighter, 0, 0, Hash40::new("head"), 6.0, -2.75, 2.75, 2.25, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
		}
		search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_N_FLAG_EAT_CHECK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn wario_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 70, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("bust"), 10.0, 45, 70, 0, 100, 6.0, -6.0, 15.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		macros::ATTACK_IGNORE_THROW(fighter, 1, 0, Hash40::new("bust"), 10.0, 45, 70, 0, 100, 6.0, -4.0, 11.5, -2.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
		macros::ATTACK_IGNORE_THROW(fighter, 2, 0, Hash40::new("bust"), 10.0, 45, 70, 0, 100, 6.0, -2.0, 8.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 38.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("bust"), -1.0, 15.0, -6.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("bust"), 0.5, 11.5, -4.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("bust"), 2.0, 8.0, -1.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 41.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("bust"), 5.0, 15.0, -5.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("bust"), 3.5, 11.5, -3.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 0, Hash40::new("bust"), 8.0, 15.0, -7.5);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("bust"), 5.0, 11.5, -4.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 47.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
		macros::CHECK_FINISH_CAMERA(fighter, 18, 4);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 48.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn wario_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 40, 91, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 40, 107, 0, 60, 7.0, 0.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::CHECK_FINISH_CAMERA(fighter, 16, 9);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn wario_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 90, 104, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 6.0, 90, 120, 0, 60, 6.5, 10.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::CHECK_FINISH_CAMERA(fighter, 1, 28);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn wario_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 80, 66, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 85, 0, 70, 6.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		macros::CHECK_FINISH_CAMERA(fighter, -2, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 32.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
}

unsafe extern "C" fn wario_sound_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_wario_attack03"));
		macros::PLAY_SE(fighter, Hash40::new("se_wario_attackhard_h01"));
	}
}

unsafe extern "C" fn wario_sound_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_wario_attack04"));
		macros::PLAY_SE(fighter, Hash40::new("se_wario_swing_l"));
	}
}

pub fn install() {
	Agent::new("wario")
		.expression_acmd("expression_attacklw4", wario_expression_attacklw4, Priority::Default)
		.expression_acmd("expression_speciallwlr", wario_expression_speciallwlr, Priority::Default)
		.expression_acmd("expression_specialairlwlr", wario_expression_speciallwlr, Priority::Default)
		.game_acmd("game_attack11", wario_game_attack11, Priority::Default)
		.game_acmd("game_attack12", wario_game_attack12, Priority::Default)
		.game_acmd("game_attackairb", wario_game_attackairb, Priority::Default)
		.game_acmd("game_attackairf", wario_game_attackairf, Priority::Default)
		.game_acmd("game_attackairhi", wario_game_attackairhi, Priority::Default)
		.game_acmd("game_attackairlw", wario_game_attackairlw, Priority::Default)
		.game_acmd("game_attackairn", wario_game_attackairn, Priority::Default)
		.game_acmd("game_attackdash", wario_game_attackdash, Priority::Default)
		.game_acmd("game_attackhi3", wario_game_attackhi3, Priority::Default)
		.game_acmd("game_attackhi4", wario_game_attackhi4, Priority::Default)
		.game_acmd("game_attacklw3", wario_game_attacklw3, Priority::Default)
		.game_acmd("game_attacklw4", wario_game_attacklw4, Priority::Default)
		.game_acmd("game_attacks3", wario_game_attacks3, Priority::Default)
		.game_acmd("game_attacks3hi", wario_game_attacks3hi, Priority::Default)
		.game_acmd("game_attacks3lw", wario_game_attacks3lw, Priority::Default)
		.game_acmd("game_attacks4", wario_game_attacks4, Priority::Default)
		.game_acmd("game_catch", wario_game_catch, Priority::Default)
		.game_acmd("game_catchattack", wario_game_catchattack, Priority::Default)
		.game_acmd("game_catchdash", wario_game_catchdash, Priority::Default)
		.game_acmd("game_cliffattack", wario_game_cliffattack, Priority::Default)
		.game_acmd("game_finaldash", wario_game_finaldash, Priority::Default)
		.game_acmd("game_finaldashend", wario_game_finaldashend, Priority::Default)
		.game_acmd("game_finalairdashend", wario_game_finaldashend, Priority::Default)
		.game_acmd("game_finalend", wario_game_finalend, Priority::Default)
		.game_acmd("game_finalairend", wario_game_finalend, Priority::Default)
		.game_acmd("game_finalvisualscene", wario_game_finalvisualscene, Priority::Default)
		.game_acmd("game_landingairlw", wario_game_landingairlw, Priority::Default)
		.game_acmd("game_specialhijump", wario_game_specialhijump, Priority::Default)
		.game_acmd("game_speciallwflyr", wario_game_speciallwflyr, Priority::Default)
		.game_acmd("game_specialairlwflyr", wario_game_speciallwflyr, Priority::Default)
		.game_acmd("game_speciallwlr", wario_game_speciallwlr, Priority::Default)
		.game_acmd("game_specialairlwlr", wario_game_speciallwlr, Priority::Default)
		.game_acmd("game_speciallwmr", wario_game_speciallwmr, Priority::Default)
		.game_acmd("game_specialairlwmr", wario_game_speciallwmr, Priority::Default)
		.game_acmd("game_speciallwsr", wario_game_speciallwsr, Priority::Default)
		.game_acmd("game_specialairlwsr", wario_game_speciallwsr, Priority::Default)
		.game_acmd("game_specialnbite", wario_game_specialnbite, Priority::Default)
		.game_acmd("game_specialairnbite", wario_game_specialnbite, Priority::Default)
		.game_acmd("game_specialnopenwait", wario_game_specialnopenwait, Priority::Default)
		.game_acmd("game_specialairnopenwait", wario_game_specialnopenwait, Priority::Default)
		.game_acmd("game_throwb", wario_game_throwb, Priority::Default)
		.game_acmd("game_throwf", wario_game_throwf, Priority::Default)
		.game_acmd("game_throwhi", wario_game_throwhi, Priority::Default)
		.game_acmd("game_throwlw", wario_game_throwlw, Priority::Default)
		.sound_acmd("sound_attackhi3", wario_sound_attackhi3, Priority::Default)
		.sound_acmd("sound_attacks3", wario_sound_attacks3, Priority::Default)
		.sound_acmd("sound_attacks3hi", wario_sound_attacks3, Priority::Default)
		.sound_acmd("sound_attacks3lw", wario_sound_attacks3, Priority::Default)
		.install();
}
