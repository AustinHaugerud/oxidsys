use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyStackGetTroopIdOp;

const DOC: &str = "Extracts troop type of the specified troop stack.";

pub const OP_CODE: u32 = 1652;

pub const IDENT: &str = "party_stack_get_troop_id";

impl Operation for PartyStackGetTroopIdOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
                make_param_doc("<stack_no>", ""),
            ],
        }
    }
}
