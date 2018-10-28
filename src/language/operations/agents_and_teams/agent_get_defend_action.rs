use language::operations::Operation;

pub struct AgentGetDefendActionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1764;

pub const IDENT: &str = "agent_get_defend_action";

impl Operation for AgentGetDefendActionOp {
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
