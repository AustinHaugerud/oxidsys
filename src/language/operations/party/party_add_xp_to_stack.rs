use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyAddXpToStackOp;

const DOC: &str = "Awards specified number of xp points to a single troop stack in the party.";

pub const OP_CODE: u32 = 1670;

pub const IDENT: &str = "party_add_xp_to_stack";

impl Operation for PartyAddXpToStackOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<party_id>", ""),
                make_param_doc("<stack_no>", ""),
                make_param_doc("<xp_amount>", ""),
            ],
        }
    }
}
