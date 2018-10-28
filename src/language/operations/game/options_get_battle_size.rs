use language::operations::Operation;

pub struct OptionsGetBattleSizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 270;

pub const IDENT: &str = "options_get_battle_size";

impl Operation for OptionsGetBattleSizeOp {
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
