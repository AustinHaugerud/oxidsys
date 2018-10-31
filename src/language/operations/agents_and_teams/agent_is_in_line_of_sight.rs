use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentIsInLineOfSightOp;

const DOC : &str = "Version 1.153+. Checks that the agent can be seen from specified position. Rotation of position register is not used (i.e. agent will be seen even if position is \"looking\" the other way).";

pub const OP_CODE: u32 = 1826;

pub const IDENT: &str = "agent_is_in_line_of_sight";

impl Operation for AgentIsInLineOfSightOp {
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
                make_param_doc("<position_no>", ""),
            ],
        }
    }
}
