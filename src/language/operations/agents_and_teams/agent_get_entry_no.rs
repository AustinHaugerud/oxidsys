use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetEntryNoOp;

const DOC : &str = "Retrieves the entry point number where this agent has spawned. What does this return for agents spawned with (spawn_agent)? 4research.";

pub const OP_CODE: u32 = 1717;

pub const IDENT: &str = "agent_get_entry_no";

impl Operation for AgentGetEntryNoOp {
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
