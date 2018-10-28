use language::operations::Operation;

pub struct OptionsGetDamageToPlayerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 260;

pub const IDENT: &str = "options_get_damage_to_player";

impl Operation for OptionsGetDamageToPlayerOp {
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
