use language::operations::Operation;

pub struct AgentSetReloadSpeedModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2094;

pub const IDENT: &str = "agent_set_reload_speed_modifier";

impl Operation for AgentSetReloadSpeedModifierOp {
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
