use {
	smash::{
		app::{
			AttackHeight,
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

#[acmd_script(agent = "zelda", script = "effect_attacklw3", category = ACMD_EFFECT)]
unsafe fn zelda_effect_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
		macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 4.6, -6, 14, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
}

#[acmd_script(agent = "zelda", script = "expression_attackhi3", category = ACMD_EXPRESSION)]
unsafe fn zelda_expression_attackhi3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

#[acmd_script(agent = "zelda", script = "expression_attackhi4", category = ACMD_EXPRESSION)]
unsafe fn zelda_expression_attackhi4(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 24.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 57.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
	}
}

#[acmd_script(agent = "zelda", scripts = ["expression_attacks3", "expression_attacks3hi", "expression_attacks3lw"], category = ACMD_EXPRESSION)]
unsafe fn zelda_expression_attacks3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
	}
}

#[acmd_script(agent = "zelda", script = "game_attack11", category = ACMD_GAME)]
unsafe fn zelda_game_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.3333333);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.5, 4.0, Some(0.0), Some(5.5), Some(4.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.5, 7.0, Some(0.0), Some(11.5), Some(4.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 180, 10, 0, 25, 3.5, 0.0, 11.5, 10.0, Some(0.0), Some(11.5), Some(4.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.5, 10.0, Some(0.0), Some(11.5), Some(4.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.5, 5.0, Some(0.0), Some(5.5), Some(5.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.5, 8.0, Some(0.0), Some(11.5), Some(5.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 180, 10, 0, 25, 3.5, 0.0, 11.5, 11.5, Some(0.0), Some(11.5), Some(5.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.5, 11.5, Some(0.0), Some(11.5), Some(5.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
	}
}

#[acmd_script(agent = "zelda", script = "game_attack100", category = ACMD_GAME)]
unsafe fn zelda_game_attack100(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	for _ in 0..1000000 {
		for _ in 0..8 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
				AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
				macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 8);
			}
			sv_animcmd::wait(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
				WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
			}
			sv_animcmd::wait(fighter.lua_state_agent, 1.0);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	}
}

#[acmd_script(agent = "zelda", script = "game_attack100end", category = ACMD_GAME)]
unsafe fn zelda_game_attack100end(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 140, 0, 60, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(18.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_attack100sub", category = ACMD_GAME)]
unsafe fn zelda_game_attack100sub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 6.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(15.0), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 8);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackairb", category = ACMD_GAME)]
unsafe fn zelda_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 20.0, 361, 86, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("toer"), 5.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 5.0, 361, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hip"), 5.0, 361, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new_raw(0x1484fa7486));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 0.9);
	sv_animcmd::frame(fighter.lua_state_agent, 48.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackairf", category = ACMD_GAME)]
unsafe fn zelda_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("toel"), 20.0, 361, 86, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("toel"), 5.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 5.0, 361, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("hip"), 5.0, 361, 90, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new_raw(0x148397b09f));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 46.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackairhi", category = ACMD_GAME)]
unsafe fn zelda_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 90, 72, 0, 60, 9.0, 0.0, 26.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 90, 72, 0, 60, 9.0, 0.0, 26.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 54.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackairlw", category = ACMD_GAME)]
unsafe fn zelda_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 18.0, 270, 105, 0, 20, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 6.0, 270, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.0, 270, 90, 0, 20, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 0, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 40.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackairn", category = ACMD_GAME)]
unsafe fn zelda_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 40, 0, 5.0, 0.0, 10.0, 7.6, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.3, 367, 100, 40, 0, 5.0, 0.0, 13.0, -5.0, None, None, None, 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.3, 367, 100, 40, 0, 5.0, 0.0, 10.0, 7.6, Some(0.0), Some(13.0), Some(-5.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.3, 95, 100, 70, 0, 5.0, 0.0, 10.0, 7.6, Some(0.0), Some(13.0), Some(-5.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 140, 0, 40, 5.0, 0.0, 13.5, -7.0, Some(0.0), Some(9.0), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 5.0, 361, 140, 0, 40, 6.0, 0.0, 7.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 5, 0, Hash40::new("head"), 5.0, 361, 140, 0, 40, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 38.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn zelda_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 70, 60, 0, 100, 3.0, 0.0, 9.0, 12.6, Some(0.0), Some(9.0), Some(13.2), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 70, 60, 0, 90, 5.5, 0.0, 9.0, 13.6, Some(0.0), Some(9.0), Some(7.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 70, 65, 0, 80, 5.0, 0.0, 9.0, 13.6, Some(0.0), Some(9.0), Some(7.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::clear(fighter.module_accessor, 0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn zelda_game_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 80, 105, 0, 50, 6.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 80, 105, 0, 50, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 8.0, 80, 105, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_attackhi4", category = ACMD_GAME)]
unsafe fn zelda_game_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 368, 100, 0, 0, 4.0, 0.0, 8.25, -5.0, Some(0.0), Some(8.25), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 0.0, y: 19.5}, 7.0 as u32, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	for _ in 0..12 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 4.6, 0.0, 19.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 367, 100, 40, 0, 4.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 366, 100, 60, 0, 4.6, 0.0, 19.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 366, 100, 60, 0, 4.0, 0.0, 18.0, -5.0, Some(0.0), Some(18.0), Some(5.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 1.0, 90, 100, 80, 0, 4.0, 0.0, 11.5, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 90, 205, 0, 40, 8.0, 0.0, 21.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 90, 205, 0, 40, 6.5, 0.0, 18.0, -6.0, Some(0.0), Some(18.0), Some(6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_attacklw3", category = ACMD_GAME)]
unsafe fn zelda_game_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 120, 0, 20, 2.6, 0.0, 2.0, 2.5, Some(0.0), Some(1.5), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 0.55);
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn zelda_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 20, 96, 0, 20, 4.2, 0.0, 3.0, 12.5, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 20, 96, 0, 20, 4.2, 0.0, 3.0, -11.0, Some(0.0), Some(7.0), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_attacks3", category = ACMD_GAME)]
unsafe fn zelda_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.1, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.2, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.3, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 3, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 80, 2.4, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 4, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 80, 2.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.85);
}

#[acmd_script(agent = "zelda", script = "game_attacks3hi", category = ACMD_GAME)]
unsafe fn zelda_game_attacks3hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.1, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.2, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.3, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 3, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 80, 2.4, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 4, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 80, 2.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.85);
}

#[acmd_script(agent = "zelda", script = "game_attacks3lw", category = ACMD_GAME)]
unsafe fn zelda_game_attacks3lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.1, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.2, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 72, 0, 70, 2.3, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 3, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 80, 2.4, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 4, 0, Hash40::new("handl"), 15.0, 361, 70, 0, 80, 2.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.85);
}

#[acmd_script(agent = "zelda", script = "game_attacks4", category = ACMD_GAME)]
unsafe fn zelda_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	for _ in 0..4 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 367, 100, 40, 0, 4.5, 0.0, 9.1, 17.0, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 367, 100, 40, 0, 4.5, 0.0, 9.1, 9.0, Some(0.0), Some(9.1), Some(17.0), 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 366, 100, 60, 0, 4.5, 0.0, 9.1, 9.0, Some(0.0), Some(9.1), Some(17.0), 0.8, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	}
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 145, 0, 40, 6.0, 0.0, 9.1, 10.5, Some(0.0), Some(9.1), Some(17.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda", script = "game_catch", category = ACMD_GAME)]
unsafe fn zelda_game_catch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 9.0, 2.1, Some(0.0), Some(9.0), Some(13.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "zelda", script = "game_catchattack", category = ACMD_GAME)]
unsafe fn zelda_game_catchattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 30, 0, 5.0, 0.0, 11.0, 8.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	macros::FT_MOTION_RATE(fighter,	0.5);
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
}

#[acmd_script(agent = "zelda", script = "game_catchdash", category = ACMD_GAME)]
unsafe fn zelda_game_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(13.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.5, 0.0, 9.0, 2.5, Some(0.0), Some(9.0), Some(14.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "zelda", script = "game_catchturn", category = ACMD_GAME)]
unsafe fn zelda_game_catchturn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter,	0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter,	1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 9.0, -4.0, Some(0.0), Some(9.0), Some(-19.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 9.0, -2.1, Some(0.0), Some(9.0), Some(-21.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

#[acmd_script(agent = "zelda", script = "game_specialairhi", category = ACMD_GAME)]
unsafe fn zelda_game_specialairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 12.0, 45, 95, 0, 80, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
	}
}

#[acmd_script(agent = "zelda", script = "game_specialairhistart", category = ACMD_GAME)]
unsafe fn zelda_game_specialairhistart(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 6.0, 80, 105, 0, 60, 10.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

#[acmd_script(agent = "zelda", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn zelda_game_specialhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		JostleModule::set_status(fighter.module_accessor, true);
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 12.0, 45, 95, 0, 80, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	macros::FT_MOTION_RATE(fighter, 1.25);
}

#[acmd_script(agent = "zelda", script = "game_specialhistart", category = ACMD_GAME)]
unsafe fn zelda_game_specialhistart(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 90, 0, 1, 150, 8.0, 0.0, 6.0, -4.0, Some(0.0), Some(6.0), Some(4.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

#[acmd_script(agent = "zelda", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME)]
unsafe fn zelda_game_specialn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.3333333);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_START);
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 40, 0, 8.5, 0.0, 7.0, -5.5, Some(0.0), Some(7.0), Some(5.5), 0.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.3, 160, 100, 40, 0, 8.5, 0.0, 7.0, -5.5, Some(0.0), Some(7.0), Some(5.5), 0.0, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 110, 0, 60, 8.5, 0.0, 7.0, -7.5, Some(0.0), Some(7.0), Some(7.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 43.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

#[acmd_script(agent = "zelda", script = "game_throwb", category = ACMD_GAME)]
unsafe fn zelda_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 75, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
		macros::CHECK_FINISH_CAMERA(fighter, 11, 7);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.3);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -5.0, y: 0.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		macros::REVERSE_LR(fighter);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "zelda", script = "game_throwf", category = ACMD_GAME)]
unsafe fn zelda_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 40, 40, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 17, 4);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.25);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 9.0, y: 1.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script(agent = "zelda", script = "game_throwhi", category = ACMD_GAME)]
unsafe fn zelda_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 90, 53, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 1, 25);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 10.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	macros::FT_MOTION_RATE(fighter,	0.8);
}

#[acmd_script(agent = "zelda", script = "game_throwlw", category = ACMD_GAME)]
unsafe fn zelda_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::FT_LEAVE_NEAR_OTTOTTO(fighter, 1.5, 1.5);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 80, 115, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 40, 100, 30, 0, 7.5, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		macros::ATK_POWER(fighter, 0, 0.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 2, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 5.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 51.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 52.0);
	macros::FT_MOTION_RATE(fighter,	0.625);
}

#[acmd_script(agent = "zelda", script = "sound_attackhi3", category = ACMD_SOUND)]
unsafe fn zelda_sound_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_zelda_rnd_attack"));
		macros::PLAY_SE(fighter, Hash40::new("se_zelda_attackhard_h01"));
		macros::PLAY_SE(fighter, Hash40::new("se_zelda_swing_s"));
	}
}

#[acmd_script(agent = "zelda", script = "sound_attackhi4", category = ACMD_SOUND)]
unsafe fn zelda_sound_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_03"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_zelda_attack06"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_zelda_smash_h01"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new_raw(0x12c3b6d87d));
	}
}

#[acmd_script(agent = "zelda_dein_s", script = "game_move", category = ACMD_GAME)]
unsafe fn zelda_dein_s_game_move(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 102, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 80, 102, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
		macros::AREA_WIND_2ND_RAD_arg9(fighter, 0, 2, 0.05, 200, 1, 0, 0, 12, 60);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		AreaModule::erase_wind(fighter.module_accessor, 0);
	}
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackkick", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackkick(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 45, 0, 50, 5.5, 0.0, 6.0, 14.0, Some(0.0), Some(6.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackl", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackl(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 0.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 110, 0, 5.5, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 80, 0, 7.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, 361, 65, 0, 60, 5.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 5.0, 361, 65, 0, 60, 5.6, 2.0, 0.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 5.0, 361, 65, 0, 60, 5.6, 2.0, 0.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 361, 65, 0, 60, 5.0, 0.0, 8.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackmax", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackmax(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 0.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 130, 0, 6.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 6, 100, 85, 0, 8.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 6.0, 45, 70, 0, 60, 6.0, 2.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.0, 45, 70, 0, 60, 6.0, 2.0, 1.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 6.0, 45, 70, 0, 60, 6.0, 2.0, 2.0, 16.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda_phantom", script = "game_attackpunch", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attackpunch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 0.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 60, 0, 4.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 6.0, 0.0, 7.0, 11.0, Some(0.0), Some(7.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 55, 0, 60, 5.5, 0.0, 10.0, 11.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 55, 0, 60, 4.5, 0.0, 9.5, 19.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script(agent = "zelda_phantom", script = "game_attacks", category = ACMD_GAME)]
unsafe fn zelda_phantom_game_attacks(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 0.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 100, 50, 0, 5.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, true, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 7.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, true, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 4.0, 361, 65, 0, 60, 5.8, 0.0, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 4.0, 361, 65, 0, 60, 5.8, 0.0, 0.0, 8.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 4.0, 361, 65, 0, 60, 5.8, 0.0, 0.0, 16.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 361, 65, 0, 60, 6.5, 0.0, 8.0, 10.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		zelda_effect_attacklw3,
		zelda_expression_attackhi3,
		zelda_expression_attackhi4,
		zelda_expression_attacks3,
		zelda_game_attack11,
		zelda_game_attack100,
		zelda_game_attack100end,
		zelda_game_attack100sub,
		zelda_game_attackairb,
		zelda_game_attackairf,
		zelda_game_attackairhi,
		zelda_game_attackairlw,
		zelda_game_attackairn,
		zelda_game_attackdash,
		zelda_game_attackhi3,
		zelda_game_attackhi4,
		zelda_game_attacklw3,
		zelda_game_attacklw4,
		zelda_game_attacks3,
		zelda_game_attacks3hi,
		zelda_game_attacks3lw,
		zelda_game_attacks4,
		zelda_game_catch,
		zelda_game_catchattack,
		zelda_game_catchdash,
		zelda_game_catchturn,
		zelda_game_specialairhi,
		zelda_game_specialairhistart,
		zelda_game_specialhi,
		zelda_game_specialhistart,
		zelda_game_specialn,
		zelda_game_throwb,
		zelda_game_throwf,
		zelda_game_throwhi,
		zelda_game_throwlw,
		zelda_sound_attackhi3,
		zelda_sound_attackhi4,
		zelda_dein_s_game_move,
		zelda_phantom_game_attackkick,
		zelda_phantom_game_attackl,
		zelda_phantom_game_attackmax,
		zelda_phantom_game_attackpunch,
		zelda_phantom_game_attacks,
	);
}
