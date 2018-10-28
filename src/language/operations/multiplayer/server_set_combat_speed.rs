use language::operations::Operation;

pub struct ServerSetCombatSpeedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 479;

pub const IDENT: &str = "server_set_combat_speed";

impl Operation for ServerSetCombatSpeedOp {
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
