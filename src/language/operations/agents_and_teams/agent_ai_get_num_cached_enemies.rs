use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiGetNumCachedEnemiesOp;

const DOC : &str = "Version 1.165+. Returns total number of nearby enemies as has been cached by agent AI. Enemies are numbered from nearest to farthest.";

pub const OP_CODE: u32 = 2670;

pub const IDENT: &str = "agent_ai_get_num_cached_enemies";

impl Operation for AgentAiGetNumCachedEnemiesOp {
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
                make_param_doc("<agent_no>", ""),
            ],
        }
    }
}
