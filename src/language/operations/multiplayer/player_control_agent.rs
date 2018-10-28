use language::operations::Operation;

pub struct PlayerControlAgentOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 421;

pub const IDENT: &str = "player_control_agent";

impl Operation for PlayerControlAgentOp {
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
