use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RemoveTroopsFromPrisonersOp;

const DOC: &str = "Removes prisoners from player's party.";

pub const OP_CODE: u32 = 1216;

pub const IDENT: &str = "remove_troops_from_prisoners";

impl Operation for RemoveTroopsFromPrisonersOp {
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
