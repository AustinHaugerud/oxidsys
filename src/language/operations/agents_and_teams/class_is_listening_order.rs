use language::operations::Operation;

pub struct ClassIsListeningOrderOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1775;

pub const IDENT: &str = "class_is_listening_order";

impl Operation for ClassIsListeningOrderOp {
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
