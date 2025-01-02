use {
	smash::{
		app::{
			ArticleOperationTarget,
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

unsafe extern "C" fn rosetta_effect_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
		macros::EFFECT(fighter, Hash40::new("rosetta_attack_flash1"), Hash40::new("top"), 0, 11.5, 9.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
		macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("havel"), 0, 7.5, 0, 0, 0, 0, 0.65, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
	}
}

unsafe extern "C" fn rosetta_effect_attack100end(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("rosetta_attack_flash1"), Hash40::new("top"), 0, 13, 16.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, false);
	}
}

unsafe extern "C" fn rosetta_effect_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_galaxy_arc2"), Hash40::new("rot"), 0, 0, 1.5, 0, 15, 75, 1, true);
		EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_ROSETTA
		&& PostureModule::lr(fighter.module_accessor) < 0.0 {
			macros::EFFECT_FLW_POS(fighter, Hash40::new("rosetta_blackhole"), Hash40::new("footl"), 7, 0, 0, 30, 90, 0, 1, true);
		} else {
			macros::EFFECT_FLW_POS(fighter, Hash40::new("rosetta_blackhole"), Hash40::new("footl"), 7, 0, 0, -30, 90, 0, 1, true);
		}
		macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
	}
}

unsafe extern "C" fn rosetta_effect_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_ROSETTA
	&& PostureModule::lr(fighter.module_accessor) < 0.0 {
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_flash2"), Hash40::new("throw"), 0, 0, 0, 0, 0, -5, 1, true);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_erase"), Hash40::new("throw"), 0, 0, 0, 0, 0, -5, 1, false);
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("rosetta_ring_erase"), -1);
		}
	} else {
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_flash2"), Hash40::new("throw"), 0, 0, 0, 0, 0, 25, 1, true);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_erase"), Hash40::new("throw"), 0, 0, 0, 0, 0, 25, 1, false);
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("rosetta_ring_erase"), -1);
		}
	}
}

unsafe extern "C" fn rosetta_effect_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_ROSETTA
	&& PostureModule::lr(fighter.module_accessor) < 0.0 {
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_flash3"), Hash40::new("throw"), 0, 0, 0, 0, 0, 10, 1, true);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_erase"), Hash40::new("throw"), 0, 0, 0, 0, 0, 10, 1, false);
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("rosetta_ring_erase"), -1);
		}
	} else {
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_flash3"), Hash40::new("throw"), 0, 0, 0, 0, 0, 30, 1, true);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 31.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_erase"), Hash40::new("throw"), 0, 0, 0, 0, 0, 30, 1, false);
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("rosetta_ring_erase"), -1);
		}
	}
}

unsafe extern "C" fn rosetta_effect_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_ROSETTA
	&& PostureModule::lr(fighter.module_accessor) < 0.0 {
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, -5, 1, true);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_erase"), Hash40::new("throw"), 0, 0, 0, 0, 0, -5, 1, false);
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("rosetta_ring_erase"), -1);
		}
	} else {
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 25, 1, true);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_ring_erase"), Hash40::new("throw"), 0, 0, 0, 0, 0, 25, 1, false);
			macros::EFFECT_DETACH_KIND(fighter, Hash40::new("rosetta_ring_erase"), -1);
		}
	}
}

unsafe extern "C" fn rosetta_effect_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_ROSETTA
		&& PostureModule::lr(fighter.module_accessor) < 0.0 {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_twinkle_arc"), Hash40::new("top"), 0, 8, 3, 5, 20, -5, 1, true);
		} else {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("rosetta_twinkle_arc"), Hash40::new("top"), 0, 8, 3, 5, 20, 5, 1, true);
		}
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_twinkle_arc"), false, false);
	}
}

unsafe extern "C" fn rosetta_expression_attack11(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
	}
}

unsafe extern "C" fn rosetta_expression_attack12(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
	}
}

unsafe extern "C" fn rosetta_expression_attack100end(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 15);
	}
}

unsafe extern "C" fn rosetta_expression_attackairf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_beams"), 0);
	}
}

unsafe extern "C" fn rosetta_expression_attackairn(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
}

unsafe extern "C" fn rosetta_expression_attackhi3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_beams"), 0);
	}
}

unsafe extern "C" fn rosetta_expression_attacklw3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 6);
	}
}

unsafe extern "C" fn rosetta_expression_catch(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
}

unsafe extern "C" fn rosetta_expression_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
	}
}

unsafe extern "C" fn rosetta_expression_catchturn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
	}
}

unsafe extern "C" fn rosetta_game_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.2);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 3.5, 0.0, 11.0, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 5, 0, 25, 3.5, 0.0, 11.0, 6.0, Some(0.0), Some(11.0), Some(11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 5, 0, 25, 3.5, 0.0, 11.0, 6.0, Some(0.0), Some(11.0), Some(11.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
}

unsafe extern "C" fn rosetta_game_attack12(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 10, 0, 25, 4.5, 0.0, 10.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 5, 0, 25, 4.5, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 5, 0, 25, 4.5, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
}

unsafe extern "C" fn rosetta_game_attack13(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 140, 0, 60, 5.0, 0.0, 10.0, 7.5, Some(0.0), Some(10.0), Some(15.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attack100(fighter: &mut L2CAgentBase) {
	loop {
		macros::FT_MOTION_RATE(fighter, 1.0);
		sv_animcmd::frame(fighter.lua_state_agent, 2.0);
		rosetta_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 4.0);
		rosetta_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 6.0);
		rosetta_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 8.0);
		rosetta_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 10.0);
		rosetta_game_attack100sub(fighter);
		fighter.clear_lua_stack();
		sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
}

unsafe extern "C" fn rosetta_game_attack100end(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 140, 0, 60, 5.5, 0.0, 10.5, 8.0, Some(0.0), Some(10.5), Some(20.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attack100sub(fighter: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 5.5, 0.0, 10.5, 8.0, Some(0.0), Some(10.5), Some(14.5), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.4, 361, 10, 0, 15, 8.0, 0.0, 10.0, 16.5, Some(0.0), Some(12.0), Some(16.5), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 8.0);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
	}
}

unsafe extern "C" fn rosetta_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.4);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 30, 85, 0, 40, 5.5, 0.0, 10.0, -17.0, Some(0.0), Some(10.0), Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn rosetta_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 1.5, 368, 100, 0, 0, 5.5, 1.0, -1.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 12.0, y: 8.0}, 3.0 as u32, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 1.5, 366, 100, 80, 0, 5.0, 3.0, -1.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 1.5, 368, 100, 0, 0, 5.0, 3.0, -1.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 1.5, 368, 100, 0, 0, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("footl"), 1.5, 368, 100, 0, 0, 4.5, 1.5, 3.25, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 4, 0, Hash40::new("footl"), 1.5, 368, 100, 0, 0, 4.5, 3.0, 6.5, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 5, 0, Hash40::new("footl"), 1.5, 368, 100, 0, 0, 6.0, 6.0, -1.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("footl"), &Vector2f{x: 0.0, y: 0.0}, 3.0 as u32, false);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("footl"), &Vector2f{x: 0.0, y: 0.0}, 3.0 as u32, false);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 3, Hash40::new("footl"), &Vector2f{x: 0.0, y: 0.0}, 3.0 as u32, false);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 4, Hash40::new("footl"), &Vector2f{x: 0.0, y: 0.0}, 3.0 as u32, false);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 5, Hash40::new("footl"), &Vector2f{x: 0.0, y: 0.0}, 3.0 as u32, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		AttackModule::clear(fighter.module_accessor, 1, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 3.0, 361, 160, 0, 60, 9.0, 6.0, 1.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 78.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

unsafe extern "C" fn rosetta_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, false, 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 10.0, 90, 90, 0, 60, 3.0, 0.0, 0.0, -4.0, Some(0.0), Some(0.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 7.0, 90, 95, 0, 50, 2.5, 0.0, 0.0, -4.0, Some(0.0), Some(0.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 4.0, 90, 100, 0, 20, 2.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn rosetta_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 15.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, false, 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 12.0, 270, 110, 0, 20, 3.0, 0.0, 0.0, -4.0, Some(0.0), Some(0.0), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 8.0, 270, 100, 0, 20, 2.5, 0.0, 0.0, -4.0, Some(0.0), Some(0.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 23.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 6.0, 361, 100, 0, 30, 2.0, 0.0, 0.0, -4.0, Some(0.0), Some(0.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 4.0, 361, 90, 0, 20, 1.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 33.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn rosetta_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.25);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 4.5, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 4.0, 0.0, 18.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 4.0, 0.0, 23.5, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("havel"), 0.0, 1.0, 0.0);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("havel"), 0.0, 7.5, 0.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 60, 80, 0, 50, 4.5, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 1, 0, Hash40::new("havel"), 8.0, 60, 80, 0, 50, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		macros::ATTACK(fighter, 2, 0, Hash40::new("havel"), 8.0, 60, 80, 0, 50, 4.0, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 34.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 10.0, 3.2);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 7.5, 9.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 1, Hash40::new("top"), 0.0, 9.3, 1.8);
		attack!(fighter, *MA_MSC_CMD_ATTACK_NODE, 2, Hash40::new("top"), 0.0, 7.1, 6.9);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 36.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 38.0);
	macros::FT_MOTION_RATE(fighter, 0.75);
	sv_animcmd::frame(fighter.lua_state_agent, 77.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}

unsafe extern "C" fn rosetta_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 60, 100, 40, 0, 4.0, 0.0, 4.0, 15.0, Some(0.0), Some(4.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 30, 100, 30, 0, 4.0, 0.0, 4.0, 15.0, Some(0.0), Some(4.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 65, 130, 0, 70, 7.0, 0.0, 10.5, 13.0, Some(0.0), Some(10.5), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 65, 130, 0, 70, 7.0, 0.0, 7.0, 6.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 368, 100, 0, 0, 3.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(6.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 25.0}, 5.0 as u32, false);
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, false, 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 9.0, 90, 70, 0, 80, 4.0, 0.0, 0.0, -3.5, Some(0.0), Some(0.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 7.0, 90, 70, 0, 80, 4.0, 0.0, 0.0, -3.5, Some(0.0), Some(0.0), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_RING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
	}
}

unsafe extern "C" fn rosetta_game_attackhi4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 12.0, 85, 82, 0, 70, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
		macros::ATTACK(fighter, 1, 0, Hash40::new("neck"), 12.0, 85, 82, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 6.0, 30, 105, 0, 30, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 30, 105, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 6.0, 30, 105, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 20, 125, 0, 30, 5.0, 0.0, 5.0, 18.0, Some(0.0), Some(5.0), Some(8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 20, 125, 0, 30, 5.0, 0.0, 5.0, -18.0, Some(0.0), Some(5.0), Some(-8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	macros::FT_MOTION_RATE(fighter, 1.0);
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 8.0, 361, 95, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 361, 95, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 8.0, 361, 95, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 120, 0, 30, 6.5, 0.0, 11.0, 10.0, Some(0.0), Some(11.0), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attacks4hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 120, 0, 30, 6.5, 0.0, 17.0, 10.0, Some(0.0), Some(20.0), Some(14.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_attacks4lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 120, 0, 30, 6.5, 0.0, 7.0, 10.0, Some(0.0), Some(6.0), Some(14.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_catch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 8.0, 2.1, Some(0.0), Some(8.0), Some(12.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn rosetta_game_catchattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 30, 0, 5.0, 0.0, 12.0, 8.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
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

unsafe extern "C" fn rosetta_game_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.5, 0.0, 8.0, 2.5, Some(0.0), Some(8.0), Some(14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn rosetta_game_catchturn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		GrabModule::set_rebound(fighter.module_accessor, true);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(-17.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
		macros::CATCH(fighter, 1, Hash40::new("top"), 1.9, 0.0, 8.0, -2.1, Some(0.0), Some(8.0), Some(-19.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
	}
	macros::game_CaptureCutCommon(fighter);
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(fighter.module_accessor, false);
	}
}

unsafe extern "C" fn rosetta_game_cliffattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 4.5, 0.0, 4.5, 15.0, Some(0.0), Some(4.5), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_downattackd(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.5, 0.0, 5.5, 13.5, Some(0.0), Some(5.5), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.5, 0.0, 5.5, -13.5, Some(0.0), Some(5.5), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_final(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
		macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		macros::SLOW_OPPONENT(fighter, 15.0, 70.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.5);
	if macros::is_excute(fighter) {
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) == false {
			macros::FT_SET_FINAL_FEAR_FACE(fighter, 55);
			if PostureModule::lr(fighter.module_accessor) < 0.0 {
				macros::REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final02.nuanmb"), false);
			} else {
				macros::REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
			}
		} else {
			macros::CAM_ZOOM_IN_arg5(fighter, 4.0, 0.0, 0.9, 0.0, 0.0);
			camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 13);
			camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 0, 0, 50, 0);
		}
		macros::FT_START_CUTIN(fighter);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 35.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 368, 100, 0, 0, 8.0, 0.0, 8.0, -6.0, Some(0.0), Some(8.0), Some(6.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
		AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: 35.0}, 5.0 as u32, false);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 50.0);
	if macros::is_excute(fighter) {
		macros::CAM_ZOOM_OUT(fighter);
	}
}

unsafe extern "C" fn rosetta_game_slipattack(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 4.5, 0.0, 4.5, -11.0, Some(0.0), Some(4.5), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 4.5, 0.0, 4.5, 11.5, Some(0.0), Some(4.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_game_specialhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 120);
		JostleModule::set_status(fighter.module_accessor, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
}

unsafe extern "C" fn rosetta_game_specialhiend(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 120);
		JostleModule::set_status(fighter.module_accessor, false);
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
	}
}

unsafe extern "C" fn rosetta_game_throwb(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 45, 70, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 29.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, -11, 10);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: -3.0, y: 3.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn rosetta_game_throwf(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 66, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 27.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 23, 14);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 4.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 28.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn rosetta_game_throwhi(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 70, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 12, 12);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: 3.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn rosetta_game_throwlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 80, 55, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
		macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::CHECK_FINISH_CAMERA(fighter, 11, 0);
		FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
		FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 5.0, y: -2.0, z: 0.0});
	}
	sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	macros::FT_MOTION_RATE(fighter, 0.8);
}

unsafe extern "C" fn rosetta_sound_attack11(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_swing_s"));
	}
}

unsafe extern "C" fn rosetta_sound_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_attackair_h01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_rosetta_rnd_attack"));
	}
}

unsafe extern "C" fn rosetta_sound_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_attackair_l01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_rosetta_rnd_attack"));
	}
}

unsafe extern "C" fn rosetta_sound_attackhi3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_attackhard_h01"));
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_rosetta_rnd_attack"));
	}
}

unsafe extern "C" fn rosetta_sound_attacklw3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_rosetta_rnd_attack"));
		macros::PLAY_SE(fighter, Hash40::new("se_rosetta_attackhard_l01"));
	}
}

unsafe extern "C" fn rosetta_sound_catch(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_swing_03"));
	}
}

unsafe extern "C" fn rosetta_sound_catchdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_swing_03"));
	}
}

unsafe extern "C" fn rosetta_sound_catchturn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_swing_03"));
	}
}

unsafe extern "C" fn rosetta_meteor_game_shoot(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 0, 100, 95, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_LEFT, true, f32::NAN, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, false, true, -1.0, false);
	}
}

unsafe extern "C" fn rosetta_powerstar_game_end(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		VisibilityModule::set_whole(fighter.module_accessor, false);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 130, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, true, true, -1.0, false);
		ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
	}
}

unsafe extern "C" fn rosetta_powerstar_game_start(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 366, 100, 30, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, f32::NAN, -1.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
		AttackModule::set_final_finish_cut_in(fighter.module_accessor, 0, false, true, -1.0, false);
	}
}

unsafe extern "C" fn rosetta_starpiece_game_shoot(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 45, 0, 30, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_OBJECT);
	}
}

unsafe extern "C" fn rosetta_tico_game_attack11(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::SEARCH(fighter, *WEAPON_ROSETTA_TICO_SEARCH_KIND_FREE_ATTACK_LR as u64, 0, Hash40::new("top"), 7.0, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(-3.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
		JostleModule::set_priority(fighter.module_accessor, *JOSTLE_PRI_HIGH);
		fighter.clear_lua_stack();
		lua_args!(fighter, *WEAPON_ROSETTA_TICO_KINETIC_ENERGY_ID_JOSTLE);
		sv_animcmd::CLR_SPEED(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 10, 0, 30, 2.0, 0.0, 4.0, 3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 180, 5, 0, 30, 2.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 5, 0, 30, 2.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_WORK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 20.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.5);
	}
}

unsafe extern "C" fn rosetta_tico_game_attack12(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		JostleModule::set_priority(fighter.module_accessor, *JOSTLE_PRI_HIGH);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 10, 0, 30, 2.5, 0.0, 4.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 5, 0, 30, 2.5, 0.0, 4.0, 4.0, Some(0.0), Some(4.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_WORK_FLAG_ENABLE_100);
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_WORK_FLAG_ENABLE_COMBO);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.5);
	}
}

unsafe extern "C" fn rosetta_tico_game_attack13(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		JostleModule::set_priority(fighter.module_accessor, *JOSTLE_PRI_HIGH);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 2.0, 80, 180, 0, 60, 3.5, 1.2, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 2.0, 80, 180, 0, 60, 2.5, -3.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.5);
	}
}

unsafe extern "C" fn rosetta_tico_game_attack100(fighter: &mut L2CAgentBase) {
	loop {
		if macros::is_excute(fighter) {
			MotionModule::set_rate(fighter.module_accessor, 1.0);
			JostleModule::set_priority(fighter.module_accessor, *JOSTLE_PRI_HIGH);
			AttackModule::clear_all(fighter.module_accessor);
		}
		sv_animcmd::frame(fighter.lua_state_agent, 2.0);
		rosetta_tico_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 4.0);
		rosetta_tico_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 6.0);
		rosetta_tico_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 8.0);
		rosetta_tico_game_attack100sub(fighter);
		sv_animcmd::frame(fighter.lua_state_agent, 10.0);
		rosetta_tico_game_attack100sub(fighter);
		fighter.clear_lua_stack();
		sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
		fighter.pop_lua_stack(1);
	}
}

unsafe extern "C" fn rosetta_tico_game_attack100end(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		JostleModule::set_priority(fighter.module_accessor, *JOSTLE_PRI_HIGH);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 180, 0, 60, 5.0, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
}

unsafe extern "C" fn rosetta_tico_game_attack100sub(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.2, 361, 10, 0, 15, 3.5, 0.0, 4.5, -3.0, Some(0.0), Some(4.5), Some(7.5), 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 0.5);
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_100_WORK_FLAG_CONTINUE_CHECK);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackairb(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 5.0, 30, 156, 0, 40, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackairf(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 10.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 3.0, 70, 10, 0, 80, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackairhi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 90, 60, 0, 110, 3.0, 0.0, 5.0, 3.5, Some(0.0), Some(13.0), Some(3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 90, 60, 0, 110, 3.0, 0.0, 5.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 90, 60, 0, 110, 3.0, 0.0, 5.0, -3.5, Some(0.0), Some(13.0), Some(-3.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackairlw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 6.0, 45, 145, 0, 60, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackairn(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 3.5);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 3.0, 361, 145, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 3.0, 361, 145, 0, 60, 5.0, 1.5, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.65);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackdash(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 30, 42, 0, 30, 5.0, 0.0, 3.6, 1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackhi3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::SEARCH(fighter, *WEAPON_ROSETTA_TICO_SEARCH_KIND_FREE_ATTACK_LR as u64, 0, Hash40::new("top"), 7.0, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(-3.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 8.0, 90, 55, 0, 120, 4.0, 0.0, 3.0, 7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 4.0, 90, 70, 0, 90, 3.0, 0.0, 3.0, 6.0, Some(0.0), Some(11.0), Some(6.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 4.0, 90, 70, 0, 90, 3.0, 0.0, 3.0, 2.5, Some(0.0), Some(10.0), Some(2.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 2, 0, Hash40::new("rot"), 4.0, 90, 70, 0, 90, 3.0, 0.0, 3.0, -1.0, Some(0.0), Some(9.0), Some(-1.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 3.0, 90, 75, 0, 80, 4.0, 0.0, 10.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		AttackModule::clear(fighter.module_accessor, 1, false);
		AttackModule::clear(fighter.module_accessor, 2, false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attackhi4(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::SEARCH(fighter, *WEAPON_ROSETTA_TICO_SEARCH_KIND_FREE_ATTACK_LR as u64, 0, Hash40::new("top"), 7.0, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(-3.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 6.0, 85, 145, 0, 70, 5.5, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attacklw3(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::SEARCH(fighter, *WEAPON_ROSETTA_TICO_SEARCH_KIND_FREE_ATTACK_LR as u64, 0, Hash40::new("top"), 7.0, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(-3.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 30, 145, 0, 30, 4.5, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attacklw4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 4.0, 20, 205, 0, 30, 3.5, 0.0, 2.0, -15.0, Some(0.0), Some(2.0), Some(-9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 18.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 4.0, 20, 205, 0, 30, 3.5, 0.0, 2.5, 7.0, Some(0.0), Some(2.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attacks3(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 2.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 5.0);
	if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 4.0, 361, 155, 0, 50, 3.5, 4.5, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attacks4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 361, 185, 0, 30, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 185, 0, 30, 2.0, 0.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attacks4hi(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 361, 185, 0, 30, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 185, 0, 30, 2.0, 0.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_attacks4lw(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		WorkModule::on_flag(fighter.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_4_WORK_FLAG_START_SMASH_HOLD);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 361, 185, 0, 30, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 185, 0, 30, 2.0, 0.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

unsafe extern "C" fn rosetta_tico_game_specialnfly(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 45, 110, 0, 50, 3.5, 0.0, 4.0, 1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
	}
}

pub fn install() {
	Agent::new("rosetta")
		.effect_acmd("effect_attack11", rosetta_effect_attack11, Priority::Default)
		.effect_acmd("effect_attack100end", rosetta_effect_attack100end, Priority::Default)
		.effect_acmd("effect_attackairf", rosetta_effect_attackairf, Priority::Default)
		.effect_acmd("effect_attackairhi", rosetta_effect_attackairhi, Priority::Default)
		.effect_acmd("effect_attackairlw", rosetta_effect_attackairlw, Priority::Default)
		.effect_acmd("effect_attackhi3", rosetta_effect_attackhi3, Priority::Default)
		.effect_acmd("effect_attacks3", rosetta_effect_attacks3, Priority::Default)
		.expression_acmd("expression_attack11", rosetta_expression_attack11, Priority::Default)
		.expression_acmd("expression_attack12", rosetta_expression_attack12, Priority::Default)
		.expression_acmd("expression_attack100end", rosetta_expression_attack100end, Priority::Default)
		.expression_acmd("expression_attackairf", rosetta_expression_attackairf, Priority::Default)
		.expression_acmd("expression_attackairn", rosetta_expression_attackairn, Priority::Default)
		.expression_acmd("expression_attackhi3", rosetta_expression_attackhi3, Priority::Default)
		.expression_acmd("expression_attacklw3", rosetta_expression_attacklw3, Priority::Default)
		.expression_acmd("expression_catch", rosetta_expression_catch, Priority::Default)
		.expression_acmd("expression_catchdash", rosetta_expression_catchdash, Priority::Default)
		.expression_acmd("expression_catchturn", rosetta_expression_catchturn, Priority::Default)
		.game_acmd("game_attack11", rosetta_game_attack11, Priority::Default)
		.game_acmd("game_attack12", rosetta_game_attack12, Priority::Default)
		.game_acmd("game_attack13", rosetta_game_attack13, Priority::Default)
		.game_acmd("game_attack100", rosetta_game_attack100, Priority::Default)
		.game_acmd("game_attack100end", rosetta_game_attack100end, Priority::Default)
		.game_acmd("game_attack100sub", rosetta_game_attack100sub, Priority::Default)
		.game_acmd("game_attackairb", rosetta_game_attackairb, Priority::Default)
		.game_acmd("game_attackairf", rosetta_game_attackairf, Priority::Default)
		.game_acmd("game_attackairhi", rosetta_game_attackairhi, Priority::Default)
		.game_acmd("game_attackairlw", rosetta_game_attackairlw, Priority::Default)
		.game_acmd("game_attackairn", rosetta_game_attackairn, Priority::Default)
		.game_acmd("game_attackdash", rosetta_game_attackdash, Priority::Default)
		.game_acmd("game_attackhi3", rosetta_game_attackhi3, Priority::Default)
		.game_acmd("game_attackhi4", rosetta_game_attackhi4, Priority::Default)
		.game_acmd("game_attacklw3", rosetta_game_attacklw3, Priority::Default)
		.game_acmd("game_attacklw4", rosetta_game_attacklw4, Priority::Default)
		.game_acmd("game_attacks3", rosetta_game_attacks3, Priority::Default)
		.game_acmd("game_attacks4", rosetta_game_attacks4, Priority::Default)
		.game_acmd("game_attacks4hi", rosetta_game_attacks4hi, Priority::Default)
		.game_acmd("game_attacks4lw", rosetta_game_attacks4lw, Priority::Default)
		.game_acmd("game_catch", rosetta_game_catch, Priority::Default)
		.game_acmd("game_catchattack", rosetta_game_catchattack, Priority::Default)
		.game_acmd("game_catchdash", rosetta_game_catchdash, Priority::Default)
		.game_acmd("game_catchturn", rosetta_game_catchturn, Priority::Default)
		.game_acmd("game_cliffattack", rosetta_game_cliffattack, Priority::Default)
		.game_acmd("game_downattackd", rosetta_game_downattackd, Priority::Default)
		.game_acmd("game_final", rosetta_game_final, Priority::Default)
		.game_acmd("game_finalair", rosetta_game_final, Priority::Default)
		.game_acmd("game_slipattack", rosetta_game_slipattack, Priority::Default)
		.game_acmd("game_specialhi", rosetta_game_specialhi, Priority::Default)
		.game_acmd("game_specialhiend", rosetta_game_specialhiend, Priority::Default)
		.game_acmd("game_throwb", rosetta_game_throwb, Priority::Default)
		.game_acmd("game_throwf", rosetta_game_throwf, Priority::Default)
		.game_acmd("game_throwhi", rosetta_game_throwhi, Priority::Default)
		.game_acmd("game_throwlw", rosetta_game_throwlw, Priority::Default)
		.sound_acmd("sound_attack11", rosetta_sound_attack11, Priority::Default)
		.sound_acmd("sound_attackairhi", rosetta_sound_attackairhi, Priority::Default)
		.sound_acmd("sound_attackairlw", rosetta_sound_attackairlw, Priority::Default)
		.sound_acmd("sound_attackhi3", rosetta_sound_attackhi3, Priority::Default)
		.sound_acmd("sound_attacklw3", rosetta_sound_attacklw3, Priority::Default)
		.sound_acmd("sound_catch", rosetta_sound_catch, Priority::Default)
		.sound_acmd("sound_catchdash", rosetta_sound_catchdash, Priority::Default)
		.sound_acmd("sound_catchturn", rosetta_sound_catchturn, Priority::Default)
		.install();
	Agent::new("rosetta_meteor")
		.game_acmd("game_shoot", rosetta_meteor_game_shoot, Priority::Default)
		.install();
	Agent::new("rosetta_powerstar")
		.game_acmd("game_end", rosetta_powerstar_game_end, Priority::Default)
		.game_acmd("game_start", rosetta_powerstar_game_start, Priority::Default)
		.install();
	Agent::new("rosetta_starpiece")
		.game_acmd("game_shoot", rosetta_starpiece_game_shoot, Priority::Default)
		.install();
	Agent::new("rosetta_tico")
		.game_acmd("game_attack11", rosetta_tico_game_attack11, Priority::Default)
		.game_acmd("game_attack12", rosetta_tico_game_attack12, Priority::Default)
		.game_acmd("game_attack13", rosetta_tico_game_attack13, Priority::Default)
		.game_acmd("game_attack100", rosetta_tico_game_attack100, Priority::Default)
		.game_acmd("game_attack100end", rosetta_tico_game_attack100end, Priority::Default)
		.game_acmd("game_attack100sub", rosetta_tico_game_attack100sub, Priority::Default)
		.game_acmd("game_attackairb", rosetta_tico_game_attackairb, Priority::Default)
		.game_acmd("game_attackairf", rosetta_tico_game_attackairf, Priority::Default)
		.game_acmd("game_attackairhi", rosetta_tico_game_attackairhi, Priority::Default)
		.game_acmd("game_attackairlw", rosetta_tico_game_attackairlw, Priority::Default)
		.game_acmd("game_attackairn", rosetta_tico_game_attackairn, Priority::Default)
		.game_acmd("game_attackdash", rosetta_tico_game_attackdash, Priority::Default)
		.game_acmd("game_attackhi3", rosetta_tico_game_attackhi3, Priority::Default)
		.game_acmd("game_attackhi4", rosetta_tico_game_attackhi4, Priority::Default)
		.game_acmd("game_attacklw3", rosetta_tico_game_attacklw3, Priority::Default)
		.game_acmd("game_attacklw4", rosetta_tico_game_attacklw4, Priority::Default)
		.game_acmd("game_attacks3", rosetta_tico_game_attacks3, Priority::Default)
		.game_acmd("game_attacks4", rosetta_tico_game_attacks4, Priority::Default)
		.game_acmd("game_attacks4hi", rosetta_tico_game_attacks4hi, Priority::Default)
		.game_acmd("game_attacks4lw", rosetta_tico_game_attacks4lw, Priority::Default)
		.game_acmd("game_specialnfly", rosetta_tico_game_specialnfly, Priority::Default)
		.install();
}
