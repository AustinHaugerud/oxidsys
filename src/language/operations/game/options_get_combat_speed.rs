use language::operations::Operation;

pub struct OptionsGetCombatSpeedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 268;

pub const IDENT: &str = "options_get_combat_speed";

impl Operation for OptionsGetCombatSpeedOp {
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
