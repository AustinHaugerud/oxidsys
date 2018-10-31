use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiSetInteractWithPlayerOp;

const DOC : &str = "Version 1.165+. Enables or disables agent AI interation with player. Dialog? Combat? 4research.";

pub const OP_CODE: u32 = 2077;

pub const IDENT: &str = "agent_ai_set_interact_with_player";

impl Operation for AgentAiSetInteractWithPlayerOp {
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
                make_param_doc("<agent_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}
