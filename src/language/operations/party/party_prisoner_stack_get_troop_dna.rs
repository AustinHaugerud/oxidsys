use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartyPrisonerStackGetTroopDnaOp;

const DOC : &str = "Extracts DNA from the specified prisoner stack. Used to properly generate appereance in conversations.";

pub const OP_CODE: u32 = 1658;

pub const IDENT: &str = "party_prisoner_stack_get_troop_dna";

impl Operation for PartyPrisonerStackGetTroopDnaOp {
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
