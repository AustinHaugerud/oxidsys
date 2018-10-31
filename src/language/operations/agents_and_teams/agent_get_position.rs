use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetPositionOp;

const DOC: &str = "Retrieves the position of the specified agent on the scene.";

pub const OP_CODE: u32 = 1710;

pub const IDENT: &str = "agent_get_position";

impl Operation for AgentGetPositionOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}
