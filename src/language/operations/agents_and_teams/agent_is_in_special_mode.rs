use language::operations::Operation;

pub struct AgentIsInSpecialModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1693;

pub const IDENT: &str = "agent_is_in_special_mode";

impl Operation for AgentIsInSpecialModeOp {
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
