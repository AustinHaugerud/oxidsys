use language::operations::Operation;

pub struct OptionsGetCombatAiOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 264;

pub const IDENT: &str = "options_get_combat_ai";

impl Operation for OptionsGetCombatAiOp {
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
