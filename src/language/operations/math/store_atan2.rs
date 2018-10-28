use language::operations::Operation;

pub struct StoreAtan2Op;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2143;

pub const IDENT: &str = "store_atan2";

impl Operation for StoreAtan2Op {
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
