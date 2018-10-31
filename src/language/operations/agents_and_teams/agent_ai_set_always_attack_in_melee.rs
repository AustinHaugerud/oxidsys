use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiSetAlwaysAttackInMeleeOp;

const DOC : &str = "Forces the agent to continuously attack in melee combat, instead of defending. Used in Native to prevent stalling at the top of the siege ladder. Use value = 0 to clear this mode.";

pub const OP_CODE: u32 = 1737;

pub const IDENT: &str = "agent_ai_set_always_attack_in_melee";

impl Operation for AgentAiSetAlwaysAttackInMeleeOp {
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
