use language::operations::Operation;

pub struct BanPlayerUsingSavedBanInfoOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 468;

pub const IDENT: &str = "ban_player_using_saved_ban_info";

impl Operation for BanPlayerUsingSavedBanInfoOp {
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
