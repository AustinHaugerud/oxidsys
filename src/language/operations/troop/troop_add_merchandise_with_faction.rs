use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopAddMerchandiseWithFactionOp;

const DOC : &str = "Same as (troop_add_merchandise), but with additional filter: only adds items which belong to specified faction, or without any factions at all.";

pub const OP_CODE: u32 = 1513;

pub const IDENT: &str = "troop_add_merchandise_with_faction";

impl Operation for TroopAddMerchandiseWithFactionOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<faction_id>", ""),
                make_param_doc("<item_type_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
