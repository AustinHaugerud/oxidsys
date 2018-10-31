use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetPartyAiBehaviorOp;

const DOC: &str = "Retrieves current AI behavior pattern for the party.";

pub const OP_CODE: u32 = 2290;

pub const IDENT: &str = "get_party_ai_behavior";

impl Operation for GetPartyAiBehaviorOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
