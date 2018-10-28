use language::operations::Operation;

pub struct TalkInfoSetLineOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2022;

pub const IDENT: &str = "talk_info_set_line";

impl Operation for TalkInfoSetLineOp {
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
