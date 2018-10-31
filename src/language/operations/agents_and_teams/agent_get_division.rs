use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetDivisionOp;

const DOC: &str = "Retrieves the agent division (custom troop class number in 0..8 range).";

pub const OP_CODE: u32 = 1773;

pub const IDENT: &str = "agent_get_division";

impl Operation for AgentGetDivisionOp {
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
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
