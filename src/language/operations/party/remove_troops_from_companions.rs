use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemoveTroopsFromCompanionsOp;

const DOC : &str = "Removes troops from player's party, duplicating functionality of (party_remove_members) but providing less flexibility.";

pub const OP_CODE: u32 = 1215;

pub const IDENT: &str = "remove_troops_from_companions";

impl Operation for RemoveTroopsFromCompanionsOp {
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
