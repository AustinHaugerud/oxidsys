use language::operations::Operation;

pub struct StoreZoomAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2220;

pub const IDENT: &str = "store_zoom_amount";

impl Operation for StoreZoomAmountOp {
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
