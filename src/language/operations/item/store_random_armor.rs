use language::operations::Operation;

pub struct StoreRandomArmorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2259;

pub const IDENT: &str = "store_random_armor";

impl Operation for StoreRandomArmorOp {
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
