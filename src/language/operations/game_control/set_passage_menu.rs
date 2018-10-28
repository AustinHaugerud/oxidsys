use language::operations::Operation;

pub struct SetPassageMenuOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1304;

pub const IDENT: &str = "set_passage_menu";

impl Operation for SetPassageMenuOp {
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
