use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopRemoveGoldOp;

const DOC: &str = "Removes gold from troop. Generally used with player or hero troops.";

pub const OP_CODE: u32 = 1529;

pub const IDENT: &str = "troop_remove_gold";

impl Operation for TroopRemoveGoldOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
