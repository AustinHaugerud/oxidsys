use language::operations::Operation;

pub struct IsPresentationActiveOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 903;

pub const IDENT: &str = "is_presentation_active";

impl Operation for IsPresentationActiveOp {
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
