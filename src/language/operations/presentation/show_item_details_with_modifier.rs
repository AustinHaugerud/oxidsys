use language::operations::Operation;

pub struct ShowItemDetailsWithModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 972;

pub const IDENT: &str = "show_item_details_with_modifier";

impl Operation for ShowItemDetailsWithModifierOp {
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
