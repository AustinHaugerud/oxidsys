use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopLootTroopOp;

const DOC : &str = "Adds to target_troop's inventory some items from source_troop's equipment and inventory with some probability. Does not actually remove items from source_troop. Commonly used in Native to generate random loot after the battle.";

pub const OP_CODE: u32 = 1539;

pub const IDENT: &str = "troop_loot_troop";

impl Operation for TroopLootTroopOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<target_troop>", ""),
                make_param_doc("<source_troop_id>", ""),
                make_param_doc("<probability>", ""),
            ],
        }
    }
}
