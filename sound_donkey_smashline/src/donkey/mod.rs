use {
	smash::{
		app::sv_animcmd,
		lua2cpp::L2CAgentBase,
		phx::Hash40
	},
	smash_script::*,
	smashline::*
};

unsafe extern "C" fn donkey_sound_batswing4(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_donkey_attack07"));
	}
}

unsafe extern "C" fn donkey_sound_itemheavyget(fighter: &mut L2CAgentBase) {
	sv_animcmd::frame(fighter.lua_state_agent, 6.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("vc_donkey_heavyget"));
	}
}

unsafe extern "C" fn donkey_sound_itemswing4(fighter: &mut L2CAgentBase) {
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
	Agent::new("donkey")
		.sound_acmd("sound_batswing4", donkey_sound_batswing4)
		.sound_acmd("sound_itemheavyget", donkey_sound_itemheavyget)
		.sound_acmd("sound_lipstickswing4", donkey_sound_itemswing4)
		.sound_acmd("sound_starrodswing4", donkey_sound_itemswing4)
		.install();
}
