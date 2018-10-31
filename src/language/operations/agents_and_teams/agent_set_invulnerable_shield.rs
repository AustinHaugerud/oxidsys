use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetInvulnerableShieldOp;

const DOC : &str = "Makes the agent invulnerable to any damage (value = 1) or makes him vulnerable again (value = 0).";

pub const OP_CODE: u32 = 1725;

pub const IDENT: &str = "agent_set_invulnerable_shield";

impl Operation for AgentSetInvulnerableShieldOp {
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
                make_param_doc("<value>", ""),
            ],
        }
    }
}
