use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod add_faction_note_from_dialog;
pub mod add_faction_note_from_sreg;
pub mod add_faction_note_tableau_mesh;
pub mod add_info_page_note_from_dialog;
pub mod add_info_page_note_from_sreg;
pub mod add_info_page_note_tableau_mesh;
pub mod add_party_note_from_dialog;
pub mod add_party_note_from_sreg;
pub mod add_party_note_tableau_mesh;
pub mod add_quest_note_from_dialog;
pub mod add_quest_note_from_sreg;
pub mod add_quest_note_tableau_mesh;
pub mod add_troop_note_from_dialog;
pub mod add_troop_note_from_sreg;
pub mod add_troop_note_tableau_mesh;
pub mod faction_set_note_available;
pub mod party_set_note_available;
pub mod quest_set_note_available;
pub mod troop_set_note_available;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(
        add_faction_note_from_dialog::AddFactionNoteFromDialogOp {},
    ));
    result.push(Box::new(
        add_faction_note_from_sreg::AddFactionNoteFromSregOp {},
    ));
    result.push(Box::new(
        add_faction_note_tableau_mesh::AddFactionNoteTableauMeshOp {},
    ));
    result.push(Box::new(
        add_info_page_note_from_dialog::AddInfoPageNoteFromDialogOp {},
    ));
    result.push(Box::new(
        add_info_page_note_from_sreg::AddInfoPageNoteFromSregOp {},
    ));
    result.push(Box::new(
        add_info_page_note_tableau_mesh::AddInfoPageNoteTableauMeshOp {},
    ));
    result.push(Box::new(
        add_party_note_from_dialog::AddPartyNoteFromDialogOp {},
    ));
    result.push(Box::new(
        add_party_note_from_sreg::AddPartyNoteFromSregOp {},
    ));
    result.push(Box::new(
        add_party_note_tableau_mesh::AddPartyNoteTableauMeshOp {},
    ));
    result.push(Box::new(
        add_quest_note_from_dialog::AddQuestNoteFromDialogOp {},
    ));
    result.push(Box::new(
        add_quest_note_from_sreg::AddQuestNoteFromSregOp {},
    ));
    result.push(Box::new(
        add_quest_note_tableau_mesh::AddQuestNoteTableauMeshOp {},
    ));
    result.push(Box::new(
        add_troop_note_from_dialog::AddTroopNoteFromDialogOp {},
    ));
    result.push(Box::new(
        add_troop_note_from_sreg::AddTroopNoteFromSregOp {},
    ));
    result.push(Box::new(
        add_troop_note_tableau_mesh::AddTroopNoteTableauMeshOp {},
    ));
    result.push(Box::new(
        faction_set_note_available::FactionSetNoteAvailableOp {},
    ));
    result.push(Box::new(
        party_set_note_available::PartySetNoteAvailableOp {},
    ));
    result.push(Box::new(
        quest_set_note_available::QuestSetNoteAvailableOp {},
    ));
    result.push(Box::new(
        troop_set_note_available::TroopSetNoteAvailableOp {},
    ));
    result
}
