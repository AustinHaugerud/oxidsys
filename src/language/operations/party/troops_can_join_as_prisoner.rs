use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopsCanJoinAsPrisonerOp;

const DOC: &str = "Checks if player party has enough space for provided number of prisoners..";

pub const OP_CODE: u32 = 106;

pub const IDENT: &str = "troops_can_join_as_prisoner";

impl Operation for TroopsCanJoinAsPrisonerOp {
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
