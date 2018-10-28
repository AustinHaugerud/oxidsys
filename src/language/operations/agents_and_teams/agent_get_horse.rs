use language::operations::Operation;

pub struct AgentGetHorseOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1714;

pub const IDENT: &str = "agent_get_horse";

impl Operation for AgentGetHorseOp {
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
