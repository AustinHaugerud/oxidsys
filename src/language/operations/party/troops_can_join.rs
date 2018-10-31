use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopsCanJoinOp;

const DOC: &str = "Checks if player party has enough space for provided number of troops.";

pub const OP_CODE: u32 = 105;

pub const IDENT: &str = "troops_can_join";

impl Operation for TroopsCanJoinOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}
