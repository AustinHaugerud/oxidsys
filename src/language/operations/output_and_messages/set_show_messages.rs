use language::operations::Operation;

pub struct SetShowMessagesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1107;

pub const IDENT: &str = "set_show_messages";

impl Operation for SetShowMessagesOp {
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
