use language::operations::Operation;

pub struct RemoveTroopsFromPrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1216;

pub const IDENT: &str = "remove_troops_from_prisoners";

impl Operation for RemoveTroopsFromPrisonersOp {
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
