use language::operations::Operation;

pub struct StoreAttributeLevelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2172;

pub const IDENT: &str = "store_attribute_level";

impl Operation for StoreAttributeLevelOp {
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
