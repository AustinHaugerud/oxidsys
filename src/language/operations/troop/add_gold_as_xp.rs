use language::operations::Operation;

pub struct AddGoldAsXpOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1063;

pub const IDENT: &str = "add_gold_as_xp";

impl Operation for AddGoldAsXpOp {
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
