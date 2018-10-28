use language::operations::Operation;

pub struct AgentSetStandAnimationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1741;

pub const IDENT: &str = "agent_set_stand_animation";

impl Operation for AgentSetStandAnimationOp {
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
