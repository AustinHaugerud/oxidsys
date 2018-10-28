use language::operations::Operation;

pub struct StoreNumPartiesCreatedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2300;

pub const IDENT: &str = "store_num_parties_created";

impl Operation for StoreNumPartiesCreatedOp {
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
