use language::operations::Operation;

pub struct ItemGetDifficultyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2702;

pub const IDENT: &str = "item_get_difficulty";

impl Operation for ItemGetDifficultyOp {
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
