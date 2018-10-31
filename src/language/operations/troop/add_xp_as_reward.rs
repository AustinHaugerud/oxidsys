use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddXpAsRewardOp;

const DOC: &str =
    "Adds the specified amount of xp points to player. Typically used as a quest reward operation.";

pub const OP_CODE: u32 = 1064;

pub const IDENT: &str = "add_xp_as_reward";

impl Operation for AddXpAsRewardOp {
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
