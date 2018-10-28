use language::operations::Operation;

pub struct PlayerIsBusyWithMenusOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 438;

pub const IDENT: &str = "player_is_busy_with_menus";

impl Operation for PlayerIsBusyWithMenusOp {
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
