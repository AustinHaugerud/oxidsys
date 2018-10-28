use language::operations::Operation;

pub struct AgentSetNoDeathKnockDownOnlyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1733;

pub const IDENT: &str = "agent_set_no_death_knock_down_only";

impl Operation for AgentSetNoDeathKnockDownOnlyOp {
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
