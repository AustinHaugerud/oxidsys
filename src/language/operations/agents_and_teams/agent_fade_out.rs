use language::operations::Operation;

pub struct AgentFadeOutOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1749;

pub const IDENT: &str = "agent_fade_out";

impl Operation for AgentFadeOutOp {
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
