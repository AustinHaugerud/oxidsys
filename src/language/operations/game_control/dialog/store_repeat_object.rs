use language::operations::Operation;

pub struct StoreRepeatObjectOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 50;

pub const IDENT: &str = "store_repeat_object";

impl Operation for StoreRepeatObjectOp {
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
