use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopAddGoldOp;

const DOC: &str = "Adds gold to troop. Generally used with player or hero troops.";

pub const OP_CODE: u32 = 1528;

pub const IDENT: &str = "troop_add_gold";

impl Operation for TroopAddGoldOp {
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
