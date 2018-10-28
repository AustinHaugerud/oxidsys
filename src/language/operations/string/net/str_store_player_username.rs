use language::operations::Operation;

pub struct StrStorePlayerUsernameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2350;

pub const IDENT: &str = "str_store_player_username";

impl Operation for StrStorePlayerUsernameOp {
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
