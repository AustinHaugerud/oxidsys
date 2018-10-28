use language::operations::Operation;

pub struct ShowItemDetailsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 970;

pub const IDENT: &str = "show_item_details";

impl Operation for ShowItemDetailsOp {
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
