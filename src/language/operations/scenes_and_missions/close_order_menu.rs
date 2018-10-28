use language::operations::Operation;

pub struct CloseOrderMenuOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1789;

pub const IDENT: &str = "close_order_menu";

impl Operation for CloseOrderMenuOp {
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
