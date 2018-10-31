use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiSetAggressivenessOp;

const DOC : &str = "Sets the aggressiveness parameter for agent AI to use. Default value is 100. Higher values make agent more aggressive. Actual game effects are not obvious, apparently used to speed up mob aggravation when previously neutral.";

pub const OP_CODE: u32 = 1753;

pub const IDENT: &str = "agent_ai_set_aggressiveness";

impl Operation for AgentAiSetAggressivenessOp {
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
