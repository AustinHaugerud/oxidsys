use language::operations::Operation;

pub struct CloseItemDetailsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 971;

pub const IDENT: &str = "close_item_details";

impl Operation for CloseItemDetailsOp {
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
