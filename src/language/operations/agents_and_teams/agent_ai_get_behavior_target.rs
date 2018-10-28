use language::operations::Operation;

pub struct AgentAiGetBehaviorTargetOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2082;

pub const IDENT: &str = "agent_ai_get_behavior_target";

impl Operation for AgentAiGetBehaviorTargetOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
