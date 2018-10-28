use language::operations::Operation;

pub struct OmitKeyOnceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 77;

pub const IDENT: &str = "omit_key_once";

impl Operation for OmitKeyOnceOp {
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
