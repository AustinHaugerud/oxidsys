use language::operations::Operation;

pub struct PlayerIsAdminOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 430;

pub const IDENT: &str = "player_is_admin";

impl Operation for PlayerIsAdminOp {
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
