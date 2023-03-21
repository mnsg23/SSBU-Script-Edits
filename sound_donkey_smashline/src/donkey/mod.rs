use {
	smash::{
		app::sv_animcmd,
		lua2cpp::L2CAgentBase,
		phx::Hash40
	},
	smash_script::*,
	smashline::*
};

#[acmd_script(agent = "donkey", script = "sound_batswing4", category = ACMD_SOUND)]
unsafe fn donkey_sound_batswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_donkey_attack07"));
	}
}

#[acmd_script(agent = "donkey", script = "sound_itemheavyget", category = ACMD_SOUND)]
unsafe fn donkey_sound_itemheavyget(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_donkey_heavyget"));
	}
}

#[acmd_script(agent = "donkey", scripts = ["sound_lipstickswing4", "sound_starrodswing4"], category = ACMD_SOUND)]
unsafe fn donkey_sound_itemswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
	}
	sv_animcmd::wait(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_donkey_attack05"));
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		donkey_sound_batswing4,
		donkey_sound_itemheavyget,
		donkey_sound_itemswing4,
	);
}
