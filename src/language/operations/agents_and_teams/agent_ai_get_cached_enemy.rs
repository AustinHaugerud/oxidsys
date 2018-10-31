use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiGetCachedEnemyOp;

const DOC : &str = "Version 1.165+. Return agent reference from AI's list of cached enemies, from nearest to farthest. Returns -1 if the cached enemy is not active anymore.";

pub const OP_CODE: u32 = 2671;

pub const IDENT: &str = "agent_ai_get_cached_enemy";

impl Operation for AgentAiGetCachedEnemyOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_no>", ""),
                make_param_doc("<cache_index>", ""),
            ],
        }
    }
}
