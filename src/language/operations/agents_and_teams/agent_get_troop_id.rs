use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetTroopIdOp;

const DOC : &str = "Retrieves the troop type of the specified agent. Returns -1 for horses (because horses are items, not troops).";

pub const OP_CODE: u32 = 1718;

pub const IDENT: &str = "agent_get_troop_id";

impl Operation for AgentGetTroopIdOp {
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
