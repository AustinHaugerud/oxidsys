use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PartySetAiBehaviorOp;

const DOC: &str = "Sets AI behavior for the party. See header_parties.py for reference.";

pub const OP_CODE: u32 = 1640;

pub const IDENT: &str = "party_set_ai_behavior";

impl Operation for PartySetAiBehaviorOp {
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
                make_param_doc("<ai_bhvr>", ""),
            ],
        }
    }
}
