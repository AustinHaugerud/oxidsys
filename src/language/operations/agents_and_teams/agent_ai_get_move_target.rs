use language::operations::Operation;

pub struct AgentAiGetMoveTargetOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2081;

pub const IDENT: &str = "agent_ai_get_move_target";

impl Operation for AgentAiGetMoveTargetOp {
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
