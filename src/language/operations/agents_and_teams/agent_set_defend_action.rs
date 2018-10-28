use language::operations::Operation;

pub struct AgentSetDefendActionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1746;

pub const IDENT: &str = "agent_set_defend_action";

impl Operation for AgentSetDefendActionOp {
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
