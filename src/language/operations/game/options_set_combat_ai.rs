use language::operations::Operation;

pub struct OptionsSetCombatAiOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 265;

pub const IDENT: &str = "options_set_combat_ai";

impl Operation for OptionsSetCombatAiOp {
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
