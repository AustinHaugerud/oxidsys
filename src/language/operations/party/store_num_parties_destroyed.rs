use language::operations::Operation;

pub struct StoreNumPartiesDestroyedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2301;

pub const IDENT: &str = "store_num_parties_destroyed";

impl Operation for StoreNumPartiesDestroyedOp {
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
