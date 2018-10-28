use language::operations::Operation;

pub struct AgentSetAnimationProgressOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1743;

pub const IDENT: &str = "agent_set_animation_progress";

impl Operation for AgentSetAnimationProgressOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
