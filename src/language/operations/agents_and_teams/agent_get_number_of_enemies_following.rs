use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetNumberOfEnemiesFollowingOp;

const DOC : &str = "Retrieves the total number of enemies who are currently attacking the specified agents. May be used for AI decision-making.";

pub const OP_CODE: u32 = 1761;

pub const IDENT: &str = "agent_get_number_of_enemies_following";

impl Operation for AgentGetNumberOfEnemiesFollowingOp {
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
