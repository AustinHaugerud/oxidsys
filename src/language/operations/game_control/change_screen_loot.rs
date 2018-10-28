use language::operations::Operation;

pub struct ChangeScreenLootOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2041;

pub const IDENT: &str = "change_screen_loot";

impl Operation for ChangeScreenLootOp {
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
