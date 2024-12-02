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

unsafe extern "C" fn dedede_expression_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

unsafe extern "C" fn dedede_expression_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

unsafe extern "C" fn dedede_expression_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

unsafe extern "C" fn dedede_expression_attacks3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 3);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 10, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
}

unsafe extern "C" fn dedede_expression_attacks4(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 34.0);
	sv_animcmd::execute(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 43.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 44.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 47.0);
	if macros::is_excute(fighter) {
		macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 76.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 14);
	}
}

unsafe extern "C" fn dedede_game_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.375);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 10, 0, 30, 3.0, 0.0, 6.5, 14.5, Some(0.0), Some(6.5), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 10, 0, 30, 3.0, 0.0, 6.5, 19.0, Some(0.0), Some(6.5), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 10, 0, 30, 3.0, 0.0, 6.5, 19.0, Some(0.0), Some(6.5), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
	}
}

unsafe extern "C" fn dedede_game_attack12(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.375);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 10, 0, 30, 4.0, 0.0, 6.5, 16.0, Some(0.0), Some(6.5), Some(3.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 10, 0, 30, 4.0, 0.0, 6.5, 22.0, Some(0.0), Some(6.5), Some(3.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 10, 0, 30, 4.0, 0.0, 6.5, 22.0, Some(0.0), Some(6.5), Some(3.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
}

unsafe extern "C" fn dedede_game_attack100(fighter: &mut L2CAgentBase) {
	loop {
		sv_animcmd::frame(fighter.lua_state_agent, 2.0);
		dedede_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 5.0);
		dedede_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 8.0);
		dedede_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 11.0);
		dedede_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 14.0);
		dedede_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 17.0);
		dedede_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 20.0);
		dedede_game_attack100sub(fighter);
		fighter.clear_lua_stack();
		sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
}

unsafe extern "C" fn dedede_game_attack100end(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 122, 0, 100, 7.0, 0.0, 13.0, 17.0, Some(0.0), Some(7.5), Some(17.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 122, 0, 100, 7.0, 0.0, 13.0, 24.0, Some(0.0), Some(7.5), Some(24.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
}

unsafe extern "C" fn dedede_game_attack100sub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 361, 10, 0, 15, 3.5, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 361, 10, 0, 15, 7.0, 0.0, 7.5, 22.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 8.0);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
	}
}

unsafe extern "C" fn dedede_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 19.0, 361, 72, 0, 60, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 19.0, 361, 72, 0, 60, 6.5, -12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 19.0, 361, 72, 0, 60, 6.5, 0.0, 10.0, -18.0, Some(0.0), Some(10.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn dedede_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 15.0, 361, 73, 0, 60, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 15.0, 361, 73, 0, 60, 5.5, -12.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn dedede_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	for _ in 0..7 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 1.0, 367, 100, 50, 0, 8.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
			macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 1.0, 90, 100, 70, 0, 8.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
			macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 1.0, 90, 100, 70, 0, 5.5, 9.0, 0.0, 0.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
			macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 1.0, 90, 100, 70, 0, 5.5, 3.0, 0.0, 0.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 5.0, 90, 145, 0, 60, 8.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 5.0, 90, 145, 0, 60, 5.5, 9.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 5.0, 90, 145, 0, 60, 5.5, 3.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
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

unsafe extern "C" fn dedede_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 19.0, 270, 100, 0, 20, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 19.0, 270, 100, 0, 20, 5.5, -12.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 44.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn dedede_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.6);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 85, 0, 40, 10.0, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 85, 0, 40, 8.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn dedede_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 60, 7.0, 0.0, 5.0, 11.8, Some(0.0), Some(5.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 6.0, 0.0, 4.0, 11.8, Some(0.0), Some(4.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 36.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 4.5, 0.0, 3.5, 10.5, Some(0.0), Some(3.5), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 39.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 80, 0, 60, 3.0, 0.0, 3.0, 9.5, Some(0.0), Some(3.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 42.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 7.0, 6.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 12.0, 85, 95, 0, 60, 6.5, -1.5, 1.0, 1.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 12.0, 85, 95, 0, 60, 6.5, 4.8, 0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
		FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 6.0);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 20.0, 85, 78, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 20.0, 85, 78, 0, 50, 4.0, -10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 20.0, 85, 78, 0, 50, 4.0, -14.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hammer2"), 20.0, 85, 78, 0, 50, 4.0, -18.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 20.0, 85, 78, 0, 50, 8.0, 0.0, 7.0, 6.0, Some(0.0), Some(7.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 4, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 75, 0, 70, 7.0, 0.0, 6.5, 12.0, Some(0.0), Some(6.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 75, 0, 70, 9.5, 0.0, 9.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 75, 0, 70, 6.0, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 361, 75, 0, 70, 8.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 16.0, 30, 79, 0, 60, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 16.0, 30, 79, 0, 60, 4.0, -7.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 16.0, 30, 79, 0, 60, 4.0, -12.5, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hammer2"), 16.0, 30, 79, 0, 60, 4.0, -18.0, -3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	for _ in 0..6 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 1.5, 367, 100, 40, 0, 7.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
			macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 1.5, 366, 100, 60, 0, 7.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
			macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 1.5, 0, 100, 80, 0, 3.5, -8.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 4.0, 361, 135, 0, 60, 8.5, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 4.0, 361, 135, 0, 60, 3.5, -8.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 6.0, 7.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 25.0, 361, 75, 0, 70, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 25.0, 361, 75, 0, 70, 4.0, -10.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 25.0, 361, 75, 0, 70, 4.0, -14.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hammer2"), 25.0, 361, 75, 0, 70, 4.0, -18.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 44.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 85, 60, 0, 100, 8.0, 0.0, 5.0, 5.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_catch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 5.0, 4.0, Some(0.0), Some(5.0), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 2.25, 0.0, 5.0, 1.75, Some(0.0), Some(5.0), Some(14.75), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn dedede_game_catchattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 361, 100, 30, 0, 5.5, 0.0, 10.0, 12.3, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(14.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 7.0, 2.2, Some(0.0), Some(7.0), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn dedede_game_catchturn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 4.5, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(-19.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 2.25, 0.0, 6.0, -1.75, Some(0.0), Some(6.0), Some(-22.05), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn dedede_game_finalend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::CAM_ZOOM_OUT(fighter);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 10.0, 60, 160, 0, 70, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_SET);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_END_EXIT);
	}
}

unsafe extern "C" fn dedede_game_finalstart(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 38, 0, 0, 0);
		macros::SLOW_OPPONENT(fighter, 30.0, 60.0);
	}
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) == false {
		sv_animcmd::frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::FT_SET_FINAL_FEAR_FACE(fighter, 75);
			macros::REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), false);
			macros::FT_START_CUTIN(fighter);
		}
	} else {
		if macros::is_excute(fighter) {
			macros::CAM_ZOOM_IN_arg5(fighter, 3.0, 0.0, 2.1, 0.0, 0.0);
			macros::FT_START_CUTIN(fighter);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 35.0);
		if macros::is_excute(fighter) {
			macros::CAM_ZOOM_OUT(fighter);
		}
	}
	sv_animcmd::frame(fighter.lua_state_agent, 60.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 40, 85, 0, 70, 21.0, 0.0, 12.0, 22.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 63.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_CLEAR_ATTACK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 80.0);
	if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) == false {
		if macros::is_excute(fighter) {
			camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
		}
	}
}

unsafe extern "C" fn dedede_game_finalvisualscene01(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 1.5, 361, 0, 1, 20, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 99.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 105.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 114.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 129.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 136.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 142.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 148.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 152.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 160.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 172.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 176.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 178.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
}

unsafe extern "C" fn dedede_game_finalvisualscene03(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_DEDEDE_FINAL, 0, 22.0, 361, 0, 1, 20, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(fighter.module_accessor, true, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 69.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_WORK_ID_FLAG_FINAL_ABS_HIT);
	}
}

unsafe extern "C" fn dedede_game_specialhijump(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_TURN);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 52.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
		KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: -4.0, z: 0.0});
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
		macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
		macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 270, 90, 0, 60, 10.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 270, 80, 0, 60, 10.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 60.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 10);
	}
}

unsafe extern "C" fn dedede_game_specialhilanding(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		macros::HIT_NODE(fighter, Hash40::new("virtualwaist"), *HIT_STATUS_OFF);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 90, 75, 0, 100, 11.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 60, 82, 0, 100, 11.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(-10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_RIGHT);
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR, false, 0);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_LEFT);
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR, false, 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_speciallw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 12.0, 361, 100, 0, 30, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 12.0, 361, 100, 0, 30, 6.5, -10.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 12.0, 361, 100, 0, 30, 6.5, -15.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn dedede_game_speciallwmax(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 20);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer2"), 40.0, 361, 110, 0, 20, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer2"), 40.0, 361, 110, 0, 20, 6.5, -10.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 40.0, 361, 110, 0, 20, 6.5, -15.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn dedede_game_specialsmiss(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 40, 7.5, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_specialsstart(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 40, 7.5, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_specialsstartsub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 40, 7.5, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn dedede_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.5, 50, 95, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 6.5, 50, 135, 0, 50, 6.0, 17.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 6.5, 50, 135, 0, 50, 6.0, 17.0, 0.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 6.5, 50, 135, 0, 50, 4.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 6.5, 50, 135, 0, 50, 4.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 28, 8);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 8.0, y: 0.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn dedede_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.5, 40, 90, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 6.5, 40, 125, 0, 50, 6.0, 17.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 6.5, 40, 125, 0, 50, 6.0, 17.0, 0.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 6.5, 40, 125, 0, 50, 4.0, 9.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 6.5, 40, 125, 0, 50, 4.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 20, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 0.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn dedede_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 90, 102, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 90, 120, 0, 60, 7.0, 0.0, 29.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 2, 32);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.7);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 12.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn dedede_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 80, 40, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 8, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 2.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	macros::FT_MOTION_RATE(fighter, 0.7);
}

unsafe extern "C" fn dedede_sound_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dedede_rnd_attack01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_dedede_hammer_swing_l"));
	}
}

unsafe extern "C" fn dedede_sound_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dedede_rnd_attack01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_dedede_hammer_swing_ll"));
	}
}

unsafe extern "C" fn dedede_sound_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dedede_rnd_attack01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_dedede_attackair_l01"));
	}
}

unsafe extern "C" fn dedede_gordo_game_specialsattack(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 16.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 16.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 16.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 15.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 15.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 15.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 14.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 14.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 13.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 13.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 13.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 11.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 11.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 60, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 10.0, 60, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 10.0, 60, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn dedede_gordo_game_specialsshot(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 18.0, 40, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -9, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 18.0, 40, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -9, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 18.0, 40, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -9, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 16.0, 40, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 16.0, 40, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 16.0, 40, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 14.0, 40, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 14.0, 40, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 40, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.0, 40, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.0, 40, 60, 0, 80, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 40, 60, 0, 80, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn dedede_gordo_game_specialswallstop(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 75, 50, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 10.0, 75, 50, 0, 50, 0.9, 5.3, 5.3, 0.0, Some(-5.3), Some(-5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 10.0, 75, 50, 0, 50, 0.9, 5.3, -5.3, 0.0, Some(-5.3), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn dedede_star_game_fly(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
	}
}

unsafe extern "C" fn dedede_starmissile_game_specialfly(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 70, 80, 0, 75, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
}

pub fn install() {
	Agent::new("dedede")
		.expression_acmd("expression_attackairb", dedede_expression_attackairb, Priority::Default)
		.expression_acmd("expression_attackairf", dedede_expression_attackairf, Priority::Default)
		.expression_acmd("expression_attackairlw", dedede_expression_attackairlw, Priority::Default)
		.expression_acmd("expression_attacks3", dedede_expression_attacks3, Priority::Default)
		.expression_acmd("expression_attacks4", dedede_expression_attacks4, Priority::Default)
		.game_acmd("game_attack11", dedede_game_attack11, Priority::Default)
		.game_acmd("game_attack12", dedede_game_attack12, Priority::Default)
		.game_acmd("game_attack100", dedede_game_attack100, Priority::Default)
		.game_acmd("game_attack100end", dedede_game_attack100end, Priority::Default)
		.game_acmd("game_attack100sub", dedede_game_attack100sub, Priority::Default)
		.game_acmd("game_attackairb", dedede_game_attackairb, Priority::Default)
		.game_acmd("game_attackairf", dedede_game_attackairf, Priority::Default)
		.game_acmd("game_attackairhi", dedede_game_attackairhi, Priority::Default)
		.game_acmd("game_attackairlw", dedede_game_attackairlw, Priority::Default)
		.game_acmd("game_attackairn", dedede_game_attackairn, Priority::Default)
		.game_acmd("game_attackdash", dedede_game_attackdash, Priority::Default)
		.game_acmd("game_attackhi3", dedede_game_attackhi3, Priority::Default)
		.game_acmd("game_attackhi4", dedede_game_attackhi4, Priority::Default)
		.game_acmd("game_attacklw3", dedede_game_attacklw3, Priority::Default)
		.game_acmd("game_attacklw4", dedede_game_attacklw4, Priority::Default)
		.game_acmd("game_attacks3", dedede_game_attacks3, Priority::Default)
		.game_acmd("game_attacks4", dedede_game_attacks4, Priority::Default)
		.game_acmd("game_catch", dedede_game_catch, Priority::Default)
		.game_acmd("game_catchattack", dedede_game_catchattack, Priority::Default)
		.game_acmd("game_catchdash", dedede_game_catchdash, Priority::Default)
		.game_acmd("game_catchturn", dedede_game_catchturn, Priority::Default)
		.game_acmd("game_finalend", dedede_game_finalend, Priority::Default)
		.game_acmd("game_finalairend", dedede_game_finalend, Priority::Default)
		.game_acmd("game_finalstart", dedede_game_finalstart, Priority::Default)
		.game_acmd("game_finalairstart", dedede_game_finalstart, Priority::Default)
		.game_acmd("game_finalvisualscene01", dedede_game_finalvisualscene01, Priority::Default)
		.game_acmd("game_finalvisualscene03", dedede_game_finalvisualscene03, Priority::Default)
		.game_acmd("game_specialhijump", dedede_game_specialhijump, Priority::Default)
		.game_acmd("game_specialhilandingl", dedede_game_specialhilanding, Priority::Default)
		.game_acmd("game_specialhilandingr", dedede_game_specialhilanding, Priority::Default)
		.game_acmd("game_speciallw", dedede_game_speciallw, Priority::Default)
		.game_acmd("game_specialairlw", dedede_game_speciallw, Priority::Default)
		.game_acmd("game_speciallwmax", dedede_game_speciallwmax, Priority::Default)
		.game_acmd("game_specialairlwmax", dedede_game_speciallwmax, Priority::Default)
		.game_acmd("game_specialsmiss", dedede_game_specialsmiss, Priority::Default)
		.game_acmd("game_specialairsmiss", dedede_game_specialsmiss, Priority::Default)
		.game_acmd("game_specialsstart", dedede_game_specialsstart, Priority::Default)
		.game_acmd("game_specialairsstart", dedede_game_specialsstart, Priority::Default)
		.game_acmd("game_specialsstartsub", dedede_game_specialsstartsub, Priority::Default)
		.game_acmd("game_throwb", dedede_game_throwb, Priority::Default)
		.game_acmd("game_throwf", dedede_game_throwf, Priority::Default)
		.game_acmd("game_throwhi", dedede_game_throwhi, Priority::Default)
		.game_acmd("game_throwlw", dedede_game_throwlw, Priority::Default)
		.sound_acmd("sound_attackairb", dedede_sound_attackairb, Priority::Default)
		.sound_acmd("sound_attackairf", dedede_sound_attackairf, Priority::Default)
		.sound_acmd("sound_attackairlw", dedede_sound_attackairlw, Priority::Default)
		.install();
	Agent::new("dedede_gordo")
		.game_acmd("game_specialsattack", dedede_gordo_game_specialsattack, Priority::Default)
		.game_acmd("game_specialsthrow", dedede_gordo_game_specialsattack, Priority::Default)
		.game_acmd("game_specialsshot", dedede_gordo_game_specialsshot, Priority::Default)
		.game_acmd("game_specialswallstop", dedede_gordo_game_specialswallstop, Priority::Default)
		.install();
	Agent::new("dedede_star")
		.game_acmd("game_fly", dedede_star_game_fly, Priority::Default)
		.install();
	Agent::new("dedede_starmissile")
		.game_acmd("game_specialfly", dedede_starmissile_game_specialfly, Priority::Default)
		.install();
}
