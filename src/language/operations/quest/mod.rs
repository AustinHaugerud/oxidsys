use language::operations::{make_param_doc, Operation, ParamInfo};
pub mod cancel_quest;
pub mod check_quest_active;
pub mod check_quest_concluded;
pub mod check_quest_failed;
pub mod check_quest_finished;
pub mod check_quest_succeeded;
pub mod complete_quest;
pub mod conclude_quest;
pub mod fail_quest;
pub mod quest_get_slot;
pub mod quest_set_slot;
pub mod quest_slot_eq;
pub mod quest_slot_ge;
pub mod set_quest_progression;
pub mod setup_quest_giver;
pub mod setup_quest_text;
pub mod start_quest;
pub mod store_partner_quest;
pub mod store_quest_item;
pub mod store_quest_number;
pub mod store_quest_troop;
pub mod store_random_quest_in_range;
pub mod store_random_troop_to_capture;
pub mod store_random_troop_to_raise;
pub mod succeed_quest;

pub fn load_operands() -> Vec<Box<Operation>> {
    let mut result: Vec<Box<Operation>> = vec![];
    result.push(Box::new(cancel_quest::CancelQuestOp {}));
    result.push(Box::new(check_quest_active::CheckQuestActiveOp {}));
    result.push(Box::new(check_quest_concluded::CheckQuestConcludedOp {}));
    result.push(Box::new(check_quest_failed::CheckQuestFailedOp {}));
    result.push(Box::new(check_quest_finished::CheckQuestFinishedOp {}));
    result.push(Box::new(check_quest_succeeded::CheckQuestSucceededOp {}));
    result.push(Box::new(complete_quest::CompleteQuestOp {}));
    result.push(Box::new(conclude_quest::ConcludeQuestOp {}));
    result.push(Box::new(fail_quest::FailQuestOp {}));
    result.push(Box::new(quest_get_slot::QuestGetSlotOp {}));
    result.push(Box::new(quest_set_slot::QuestSetSlotOp {}));
    result.push(Box::new(quest_slot_eq::QuestSlotEqOp {}));
    result.push(Box::new(quest_slot_ge::QuestSlotGeOp {}));
    result.push(Box::new(setup_quest_giver::SetupQuestGiverOp {}));
    result.push(Box::new(setup_quest_text::SetupQuestTextOp {}));
    result.push(Box::new(set_quest_progression::SetQuestProgressionOp {}));
    result.push(Box::new(start_quest::StartQuestOp {}));
    result.push(Box::new(store_partner_quest::StorePartnerQuestOp {}));
    result.push(Box::new(store_quest_item::StoreQuestItemOp {}));
    result.push(Box::new(store_quest_number::StoreQuestNumberOp {}));
    result.push(Box::new(store_quest_troop::StoreQuestTroopOp {}));
    result.push(Box::new(
        store_random_quest_in_range::StoreRandomQuestInRangeOp {},
    ));
    result.push(Box::new(
        store_random_troop_to_capture::StoreRandomTroopToCaptureOp {},
    ));
    result.push(Box::new(
        store_random_troop_to_raise::StoreRandomTroopToRaiseOp {},
    ));
    result.push(Box::new(succeed_quest::SucceedQuestOp {}));
    result
}
