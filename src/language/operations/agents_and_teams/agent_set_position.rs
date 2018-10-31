use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetPositionOp;

const DOC : &str = "Teleports the agent to specified position on the scene. Be careful with riders - you must teleport the horse, not the rider for the operation to work correctly!";

pub const OP_CODE: u32 = 1711;

pub const IDENT: &str = "agent_set_position";

impl Operation for AgentSetPositionOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}
