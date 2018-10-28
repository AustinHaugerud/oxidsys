use language::operations::Operation;

pub struct StoreCharacterLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2171;

pub const IDENT: &str = "store_character_level";

impl Operation for StoreCharacterLevelOp {
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
