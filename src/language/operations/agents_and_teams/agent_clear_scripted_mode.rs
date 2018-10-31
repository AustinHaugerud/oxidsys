use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentClearScriptedModeOp;

const DOC: &str = "Clears scripting mode from the agent, making him behave as usual again.";

pub const OP_CODE: u32 = 1735;

pub const IDENT: &str = "agent_clear_scripted_mode";

impl Operation for AgentClearScriptedModeOp {
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
