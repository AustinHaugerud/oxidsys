use language::operations::Operation;

pub struct AgentSetUseSpeedModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2095;

pub const IDENT: &str = "agent_set_use_speed_modifier";

impl Operation for AgentSetUseSpeedModifierOp {
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
