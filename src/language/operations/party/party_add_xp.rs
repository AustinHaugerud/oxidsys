use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyAddXpOp;

const DOC: &str =
    "Awards specified number of xp points to entire party (split between all stacks).";

pub const OP_CODE: u32 = 1674;

pub const IDENT: &str = "party_add_xp";

impl Operation for PartyAddXpOp {
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
                make_param_doc("<party_id>", ""),
                make_param_doc("<xp_amount>", ""),
            ],
        }
    }
}
