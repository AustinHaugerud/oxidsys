use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod agent_get_player_id;
pub mod ban_player;
pub mod ban_player_using_saved_ban_info;
pub mod game_in_multiplayer_mode;
pub mod get_max_players;
pub mod kick_player;
pub mod multiplayer_clear_scene;
pub mod multiplayer_find_spawn_point;
pub mod multiplayer_get_my_gold;
pub mod multiplayer_get_my_player;
pub mod multiplayer_get_my_team;
pub mod multiplayer_get_my_troop;
pub mod multiplayer_is_dedicated_server;
pub mod multiplayer_is_server;
pub mod multiplayer_make_everyone_enemy;
pub mod multiplayer_send_2_int_to_server;
pub mod multiplayer_send_3_int_to_server;
pub mod multiplayer_send_4_int_to_server;
pub mod multiplayer_send_int_to_server;
pub mod multiplayer_send_message_to_server;
pub mod multiplayer_send_string_to_server;
pub mod multiplayer_set_my_troop;
pub mod party_slot_ge;
pub mod player_add_spawn_item;
pub mod player_control_agent;
pub mod player_get_agent_id;
pub mod player_get_banner_id;
pub mod player_get_death_count;
pub mod player_get_gender;
pub mod player_get_gold;
pub mod player_get_is_muted;
pub mod player_get_item_id;
pub mod player_get_kill_count;
pub mod player_get_ping;
pub mod player_get_score;
pub mod player_get_slot;
pub mod player_get_team_no;
pub mod player_get_troop_id;
pub mod player_get_unique_id;
pub mod player_get_value_of_original_items;
pub mod player_is_active;
pub mod player_is_admin;
pub mod player_is_busy_with_menus;
pub mod player_item_slot_is_picked_up;
pub mod player_save_picked_up_items_for_next_spawn;
pub mod player_set_death_count;
pub mod player_set_gold;
pub mod player_set_is_admin;
pub mod player_set_is_muted;
pub mod player_set_kill_count;
pub mod player_set_score;
pub mod player_set_slot;
pub mod player_set_team_no;
pub mod player_set_troop_id;
pub mod player_slot_eq;
pub mod player_spawn_new_agent;
pub mod profile_get_banner_id;
pub mod profile_set_banner_id;
pub mod save_ban_info_of_player;
pub mod send_message_to_url;
pub mod server_add_message_to_log;
pub mod server_get_add_to_game_servers_list;
pub mod server_get_anti_cheat;
pub mod server_get_changing_game_type_allowed;
pub mod server_get_combat_speed;
pub mod server_get_control_block_dir;
pub mod server_get_friendly_fire;
pub mod server_get_friendly_fire_damage_friend_ratio;
pub mod server_get_friendly_fire_damage_self_ratio;
pub mod server_get_ghost_mode;
pub mod server_get_max_num_players;
pub mod server_get_melee_friendly_fire;
pub mod server_get_renaming_server_allowed;
pub mod server_set_add_to_game_servers_list;
pub mod server_set_anti_cheat;
pub mod server_set_combat_speed;
pub mod server_set_control_block_dir;
pub mod server_set_friendly_fire;
pub mod server_set_friendly_fire_damage_friend_ratio;
pub mod server_set_friendly_fire_damage_self_ratio;
pub mod server_set_ghost_mode;
pub mod server_set_max_num_players;
pub mod server_set_melee_friendly_fire;
pub mod server_set_name;
pub mod server_set_password;
pub mod server_set_welcome_message;
pub mod set_spawn_effector_scene_prop_id;
pub mod set_spawn_effector_scene_prop_kind;
pub mod start_multiplayer_mission;
pub mod team_get_bot_death_count;
pub mod team_get_bot_kill_count;
pub mod team_get_faction;
pub mod team_get_kill_count;
pub mod team_get_score;
pub mod team_set_bot_death_count;
pub mod team_set_bot_kill_count;
pub mod team_set_faction;
pub mod team_set_score;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(agent_get_player_id::AgentGetPlayerIdOp {}));
    result.push(Box::new(ban_player::BanPlayerOp {}));
    result.push(Box::new(
        ban_player_using_saved_ban_info::BanPlayerUsingSavedBanInfoOp {},
    ));
    result.push(Box::new(
        game_in_multiplayer_mode::GameInMultiplayerModeOp {},
    ));
    result.push(Box::new(get_max_players::GetMaxPlayersOp {}));
    result.push(Box::new(kick_player::KickPlayerOp {}));
    result.push(Box::new(
        multiplayer_clear_scene::MultiplayerClearSceneOp {},
    ));
    result.push(Box::new(
        multiplayer_find_spawn_point::MultiplayerFindSpawnPointOp {},
    ));
    result.push(Box::new(multiplayer_get_my_gold::MultiplayerGetMyGoldOp {}));
    result.push(Box::new(
        multiplayer_get_my_player::MultiplayerGetMyPlayerOp {},
    ));
    result.push(Box::new(multiplayer_get_my_team::MultiplayerGetMyTeamOp {}));
    result.push(Box::new(
        multiplayer_get_my_troop::MultiplayerGetMyTroopOp {},
    ));
    result.push(Box::new(
        multiplayer_is_dedicated_server::MultiplayerIsDedicatedServerOp {},
    ));
    result.push(Box::new(multiplayer_is_server::MultiplayerIsServerOp {}));
    result.push(Box::new(
        multiplayer_make_everyone_enemy::MultiplayerMakeEveryoneEnemyOp {},
    ));
    result.push(Box::new(
        multiplayer_send_2_int_to_server::MultiplayerSend2IntToServerOp {},
    ));
    result.push(Box::new(
        multiplayer_send_3_int_to_server::MultiplayerSend3IntToServerOp {},
    ));
    result.push(Box::new(
        multiplayer_send_4_int_to_server::MultiplayerSend4IntToServerOp {},
    ));
    result.push(Box::new(
        multiplayer_send_int_to_server::MultiplayerSendIntToServerOp {},
    ));
    result.push(Box::new(
        multiplayer_send_message_to_server::MultiplayerSendMessageToServerOp {},
    ));
    result.push(Box::new(
        multiplayer_send_string_to_server::MultiplayerSendStringToServerOp {},
    ));
    result.push(Box::new(
        multiplayer_set_my_troop::MultiplayerSetMyTroopOp {},
    ));
    result.push(Box::new(party_slot_ge::PartySlotGeOp {}));
    result.push(Box::new(player_add_spawn_item::PlayerAddSpawnItemOp {}));
    result.push(Box::new(player_control_agent::PlayerControlAgentOp {}));
    result.push(Box::new(player_get_agent_id::PlayerGetAgentIdOp {}));
    result.push(Box::new(player_get_banner_id::PlayerGetBannerIdOp {}));
    result.push(Box::new(player_get_death_count::PlayerGetDeathCountOp {}));
    result.push(Box::new(player_get_gender::PlayerGetGenderOp {}));
    result.push(Box::new(player_get_gold::PlayerGetGoldOp {}));
    result.push(Box::new(player_get_is_muted::PlayerGetIsMutedOp {}));
    result.push(Box::new(player_get_item_id::PlayerGetItemIdOp {}));
    result.push(Box::new(player_get_kill_count::PlayerGetKillCountOp {}));
    result.push(Box::new(player_get_ping::PlayerGetPingOp {}));
    result.push(Box::new(player_get_score::PlayerGetScoreOp {}));
    result.push(Box::new(player_get_slot::PlayerGetSlotOp {}));
    result.push(Box::new(player_get_team_no::PlayerGetTeamNoOp {}));
    result.push(Box::new(player_get_troop_id::PlayerGetTroopIdOp {}));
    result.push(Box::new(player_get_unique_id::PlayerGetUniqueIdOp {}));
    result.push(Box::new(
        player_get_value_of_original_items::PlayerGetValueOfOriginalItemsOp {},
    ));
    result.push(Box::new(player_is_active::PlayerIsActiveOp {}));
    result.push(Box::new(player_is_admin::PlayerIsAdminOp {}));
    result.push(Box::new(
        player_is_busy_with_menus::PlayerIsBusyWithMenusOp {},
    ));
    result.push(Box::new(
        player_item_slot_is_picked_up::PlayerItemSlotIsPickedUpOp {},
    ));
    result.push(Box::new(
        player_save_picked_up_items_for_next_spawn::PlayerSavePickedUpItemsForNextSpawnOp {},
    ));
    result.push(Box::new(player_set_death_count::PlayerSetDeathCountOp {}));
    result.push(Box::new(player_set_gold::PlayerSetGoldOp {}));
    result.push(Box::new(player_set_is_admin::PlayerSetIsAdminOp {}));
    result.push(Box::new(player_set_is_muted::PlayerSetIsMutedOp {}));
    result.push(Box::new(player_set_kill_count::PlayerSetKillCountOp {}));
    result.push(Box::new(player_set_score::PlayerSetScoreOp {}));
    result.push(Box::new(player_set_slot::PlayerSetSlotOp {}));
    result.push(Box::new(player_set_team_no::PlayerSetTeamNoOp {}));
    result.push(Box::new(player_set_troop_id::PlayerSetTroopIdOp {}));
    result.push(Box::new(player_slot_eq::PlayerSlotEqOp {}));
    result.push(Box::new(player_spawn_new_agent::PlayerSpawnNewAgentOp {}));
    result.push(Box::new(profile_get_banner_id::ProfileGetBannerIdOp {}));
    result.push(Box::new(profile_set_banner_id::ProfileSetBannerIdOp {}));
    result.push(Box::new(save_ban_info_of_player::SaveBanInfoOfPlayerOp {}));
    result.push(Box::new(send_message_to_url::SendMessageToUrlOp {}));
    result.push(Box::new(
        server_add_message_to_log::ServerAddMessageToLogOp {},
    ));
    result.push(Box::new(
        server_get_add_to_game_servers_list::ServerGetAddToGameServersListOp {},
    ));
    result.push(Box::new(server_get_anti_cheat::ServerGetAntiCheatOp {}));
    result.push(Box::new(
        server_get_changing_game_type_allowed::ServerGetChangingGameTypeAllowedOp {},
    ));
    result.push(Box::new(server_get_combat_speed::ServerGetCombatSpeedOp {}));
    result.push(Box::new(
        server_get_control_block_dir::ServerGetControlBlockDirOp {},
    ));
    result.push(Box::new(
        server_get_friendly_fire::ServerGetFriendlyFireOp {},
    ));
    result.push(Box::new(
        server_get_friendly_fire_damage_friend_ratio::ServerGetFriendlyFireDamageFriendRatioOp {},
    ));
    result.push(Box::new(
        server_get_friendly_fire_damage_self_ratio::ServerGetFriendlyFireDamageSelfRatioOp {},
    ));
    result.push(Box::new(server_get_ghost_mode::ServerGetGhostModeOp {}));
    result.push(Box::new(
        server_get_max_num_players::ServerGetMaxNumPlayersOp {},
    ));
    result.push(Box::new(
        server_get_melee_friendly_fire::ServerGetMeleeFriendlyFireOp {},
    ));
    result.push(Box::new(
        server_get_renaming_server_allowed::ServerGetRenamingServerAllowedOp {},
    ));
    result.push(Box::new(
        server_set_add_to_game_servers_list::ServerSetAddToGameServersListOp {},
    ));
    result.push(Box::new(server_set_anti_cheat::ServerSetAntiCheatOp {}));
    result.push(Box::new(server_set_combat_speed::ServerSetCombatSpeedOp {}));
    result.push(Box::new(
        server_set_control_block_dir::ServerSetControlBlockDirOp {},
    ));
    result.push(Box::new(
        server_set_friendly_fire::ServerSetFriendlyFireOp {},
    ));
    result.push(Box::new(
        server_set_friendly_fire_damage_friend_ratio::ServerSetFriendlyFireDamageFriendRatioOp {},
    ));
    result.push(Box::new(
        server_set_friendly_fire_damage_self_ratio::ServerSetFriendlyFireDamageSelfRatioOp {},
    ));
    result.push(Box::new(server_set_ghost_mode::ServerSetGhostModeOp {}));
    result.push(Box::new(
        server_set_max_num_players::ServerSetMaxNumPlayersOp {},
    ));
    result.push(Box::new(
        server_set_melee_friendly_fire::ServerSetMeleeFriendlyFireOp {},
    ));
    result.push(Box::new(server_set_name::ServerSetNameOp {}));
    result.push(Box::new(server_set_password::ServerSetPasswordOp {}));
    result.push(Box::new(
        server_set_welcome_message::ServerSetWelcomeMessageOp {},
    ));
    result.push(Box::new(
        set_spawn_effector_scene_prop_id::SetSpawnEffectorScenePropIdOp {},
    ));
    result.push(Box::new(
        set_spawn_effector_scene_prop_kind::SetSpawnEffectorScenePropKindOp {},
    ));
    result.push(Box::new(
        start_multiplayer_mission::StartMultiplayerMissionOp {},
    ));
    result.push(Box::new(
        team_get_bot_death_count::TeamGetBotDeathCountOp {},
    ));
    result.push(Box::new(team_get_bot_kill_count::TeamGetBotKillCountOp {}));
    result.push(Box::new(team_get_faction::TeamGetFactionOp {}));
    result.push(Box::new(team_get_kill_count::TeamGetKillCountOp {}));
    result.push(Box::new(team_get_score::TeamGetScoreOp {}));
    result.push(Box::new(
        team_set_bot_death_count::TeamSetBotDeathCountOp {},
    ));
    result.push(Box::new(team_set_bot_kill_count::TeamSetBotKillCountOp {}));
    result.push(Box::new(team_set_faction::TeamSetFactionOp {}));
    result.push(Box::new(team_set_score::TeamSetScoreOp {}));
    result
}
