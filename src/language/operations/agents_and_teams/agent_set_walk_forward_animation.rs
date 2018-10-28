use language::operations::Operation;

pub struct AgentSetWalkForwardAnimationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1742;

pub const IDENT: &str = "agent_set_walk_forward_animation";

impl Operation for AgentSetWalkForwardAnimationOp {
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
