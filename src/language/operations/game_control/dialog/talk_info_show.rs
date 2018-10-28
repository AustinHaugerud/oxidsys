use language::operations::Operation;

pub struct TalkInfoShowOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2020;

pub const IDENT: &str = "talk_info_show";

impl Operation for TalkInfoShowOp {
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
