use language::operations::Operation;

pub struct SaveBanInfoOfPlayerOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 467;

pub const IDENT: &str = "save_ban_info_of_player";

impl Operation for SaveBanInfoOfPlayerOp {
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
