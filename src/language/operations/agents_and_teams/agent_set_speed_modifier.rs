use language::operations::Operation;

pub struct AgentSetSpeedModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2093;

pub const IDENT: &str = "agent_set_speed_modifier";

impl Operation for AgentSetSpeedModifierOp {
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
