use language::operations::Operation;

pub struct PlayerSetIsAdminOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 429;

pub const IDENT: &str = "player_set_is_admin";

impl Operation for PlayerSetIsAdminOp {
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
