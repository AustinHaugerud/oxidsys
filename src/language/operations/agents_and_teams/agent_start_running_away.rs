use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentStartRunningAwayOp;

const DOC : &str = "Makes the agent flee the battlefield, ignoring everything else and not attacking. If the agent reaches the edge of map in this mode, he will fade out. Optional position_no parameter added in 1.153 and will make the agent flee to specified position instead (pos0 is not allowed and will be ignored).";

pub const OP_CODE: u32 = 1751;

pub const IDENT: &str = "agent_start_running_away";

impl Operation for AgentStartRunningAwayOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<position_no>", ""),
                make_param_doc("[<position_no>]", ""),
            ],
        }
    }
}
