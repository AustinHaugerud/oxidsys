use language::operations::Operation;

pub struct ServerGetCombatSpeedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 478;

pub const IDENT: &str = "server_get_combat_speed";

impl Operation for ServerGetCombatSpeedOp {
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
