use language::operations::Operation;

pub struct ClearOmittedKeysOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 78;

pub const IDENT: &str = "clear_omitted_keys";

impl Operation for ClearOmittedKeysOp {
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
