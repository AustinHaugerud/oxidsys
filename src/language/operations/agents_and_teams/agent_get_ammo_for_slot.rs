use language::operations::Operation;

pub struct AgentGetAmmoForSlotOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1825;

pub const IDENT: &str = "agent_get_ammo_for_slot";

impl Operation for AgentGetAmmoForSlotOp {
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
