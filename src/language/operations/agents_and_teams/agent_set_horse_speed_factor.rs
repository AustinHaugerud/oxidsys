use language::operations::Operation;

pub struct AgentSetHorseSpeedFactorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1734;

pub const IDENT: &str = "agent_set_horse_speed_factor";

impl Operation for AgentSetHorseSpeedFactorOp {
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
