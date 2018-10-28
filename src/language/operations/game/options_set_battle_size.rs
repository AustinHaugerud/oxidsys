use language::operations::Operation;

pub struct OptionsSetBattleSizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 271;

pub const IDENT: &str = "options_set_battle_size";

impl Operation for OptionsSetBattleSizeOp {
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
