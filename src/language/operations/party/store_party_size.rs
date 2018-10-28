use language::operations::Operation;

pub struct StorePartySizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2156;

pub const IDENT: &str = "store_party_size";

impl Operation for StorePartySizeOp {
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
