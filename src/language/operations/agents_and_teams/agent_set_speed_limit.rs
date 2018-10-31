use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetSpeedLimitOp;

const DOC : &str = "Limits agent speed by the specified value in kph. Use 5 for average walking speed. Affects only AI agents.";

pub const OP_CODE: u32 = 1736;

pub const IDENT: &str = "agent_set_speed_limit";

impl Operation for AgentSetSpeedLimitOp {
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
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}
