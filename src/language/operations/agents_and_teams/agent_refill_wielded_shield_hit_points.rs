use language::operations::Operation;

pub struct AgentRefillWieldedShieldHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1692;

pub const IDENT: &str = "agent_refill_wielded_shield_hit_points";

impl Operation for AgentRefillWieldedShieldHitPointsOp {
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
