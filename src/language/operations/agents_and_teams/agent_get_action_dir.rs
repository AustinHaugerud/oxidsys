use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetActionDirOp;

const DOC : &str = "Retrieves the direction of current agent's action. Possible values: invalid = -1, down = 0, right = 1, left = 2, up = 3.";

pub const OP_CODE: u32 = 1767;

pub const IDENT: &str = "agent_get_action_dir";

impl Operation for AgentGetActionDirOp {
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
