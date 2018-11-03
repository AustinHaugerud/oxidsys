use language::operations::{Operation};
pub mod change_screen_buy_mercenaries;
pub mod change_screen_controls;
pub mod change_screen_equip_other;
pub mod change_screen_exchange_members;
pub mod change_screen_exchange_with_party;
pub mod change_screen_give_members;
pub mod change_screen_loot;
pub mod change_screen_map;
pub mod change_screen_map_conversation;
pub mod change_screen_mission;
pub mod change_screen_notes;
pub mod change_screen_options;
pub mod change_screen_quit;
pub mod change_screen_return;
pub mod change_screen_trade;
pub mod change_screen_trade_prisoners;
pub mod change_screen_training;
pub mod change_screen_view_character;
pub mod conversation_screen_is_active;
pub mod dialog;
pub mod encounter_attack;
pub mod encountered_party_is_attacker;
pub mod end_current_battle;
pub mod finish_party_battle_mode;
pub mod jump_to_menu;
pub mod leave_encounter;
pub mod select_enemy;
pub mod set_background_mesh;
pub mod set_conversation_speaker_agent;
pub mod set_conversation_speaker_troop;
pub mod set_encountered_party;
pub mod set_game_menu_tableau_mesh;
pub mod set_mercenary_source_party;
pub mod set_party_battle_mode;
pub mod set_passage_menu;
pub mod start_encounter;
pub mod start_map_conversation;
pub mod start_mission_conversation;
pub mod store_conversation_agent;
pub mod store_conversation_troop;
pub mod store_encountered_party;
pub mod store_encountered_party2;
pub mod store_partner_faction;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        change_screen_buy_mercenaries::ChangeScreenBuyMercenariesOp {},
    ));
    result.push(Box::new(change_screen_controls::ChangeScreenControlsOp {}));
    result.push(Box::new(
        change_screen_equip_other::ChangeScreenEquipOtherOp {},
    ));
    result.push(Box::new(
        change_screen_exchange_members::ChangeScreenExchangeMembersOp {},
    ));
    result.push(Box::new(
        change_screen_exchange_with_party::ChangeScreenExchangeWithPartyOp {},
    ));
    result.push(Box::new(
        change_screen_give_members::ChangeScreenGiveMembersOp {},
    ));
    result.push(Box::new(change_screen_loot::ChangeScreenLootOp {}));
    result.push(Box::new(change_screen_map::ChangeScreenMapOp {}));
    result.push(Box::new(
        change_screen_map_conversation::ChangeScreenMapConversationOp {},
    ));
    result.push(Box::new(change_screen_mission::ChangeScreenMissionOp {}));
    result.push(Box::new(change_screen_notes::ChangeScreenNotesOp {}));
    result.push(Box::new(change_screen_options::ChangeScreenOptionsOp {}));
    result.push(Box::new(change_screen_quit::ChangeScreenQuitOp {}));
    result.push(Box::new(change_screen_return::ChangeScreenReturnOp {}));
    result.push(Box::new(change_screen_trade::ChangeScreenTradeOp {}));
    result.push(Box::new(
        change_screen_trade_prisoners::ChangeScreenTradePrisonersOp {},
    ));
    result.push(Box::new(change_screen_training::ChangeScreenTrainingOp {}));
    result.push(Box::new(
        change_screen_view_character::ChangeScreenViewCharacterOp {},
    ));
    result.push(Box::new(
        conversation_screen_is_active::ConversationScreenIsActiveOp {},
    ));
    result.push(Box::new(
        encountered_party_is_attacker::EncounteredPartyIsAttackerOp {},
    ));
    result.push(Box::new(encounter_attack::EncounterAttackOp {}));
    result.push(Box::new(end_current_battle::EndCurrentBattleOp {}));
    result.push(Box::new(
        finish_party_battle_mode::FinishPartyBattleModeOp {},
    ));
    result.push(Box::new(jump_to_menu::JumpToMenuOp {}));
    result.push(Box::new(leave_encounter::LeaveEncounterOp {}));
    result.push(Box::new(select_enemy::SelectEnemyOp {}));
    result.push(Box::new(set_background_mesh::SetBackgroundMeshOp {}));
    result.push(Box::new(
        set_conversation_speaker_agent::SetConversationSpeakerAgentOp {},
    ));
    result.push(Box::new(
        set_conversation_speaker_troop::SetConversationSpeakerTroopOp {},
    ));
    result.push(Box::new(set_encountered_party::SetEncounteredPartyOp {}));
    result.push(Box::new(
        set_game_menu_tableau_mesh::SetGameMenuTableauMeshOp {},
    ));
    result.push(Box::new(
        set_mercenary_source_party::SetMercenarySourcePartyOp {},
    ));
    result.push(Box::new(set_party_battle_mode::SetPartyBattleModeOp {}));
    result.push(Box::new(set_passage_menu::SetPassageMenuOp {}));
    result.push(Box::new(start_encounter::StartEncounterOp {}));
    result.push(Box::new(start_map_conversation::StartMapConversationOp {}));
    result.push(Box::new(
        start_mission_conversation::StartMissionConversationOp {},
    ));
    result.push(Box::new(
        store_conversation_agent::StoreConversationAgentOp {},
    ));
    result.push(Box::new(
        store_conversation_troop::StoreConversationTroopOp {},
    ));
    result.push(Box::new(
        store_encountered_party::StoreEncounteredPartyOp {},
    ));
    result.push(Box::new(
        store_encountered_party2::StoreEncounteredParty2Op {},
    ));
    result.push(Box::new(store_partner_faction::StorePartnerFactionOp {}));
    result.extend(dialog::load_operands());
    result
}
