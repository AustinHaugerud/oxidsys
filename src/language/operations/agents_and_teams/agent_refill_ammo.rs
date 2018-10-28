use language::operations::Operation;

pub struct AgentRefillAmmoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1728;

pub const IDENT: &str = "agent_refill_ammo";

impl Operation for AgentRefillAmmoOp {
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
