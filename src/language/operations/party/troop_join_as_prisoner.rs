use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopJoinAsPrisonerOp;

const DOC: &str = "Specified hero joins player's party as prisoner";

pub const OP_CODE: u32 = 1204;

pub const IDENT: &str = "troop_join_as_prisoner";

impl Operation for TroopJoinAsPrisonerOp {
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
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}
