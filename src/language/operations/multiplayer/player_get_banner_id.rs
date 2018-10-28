use language::operations::Operation;

pub struct PlayerGetBannerIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 423;

pub const IDENT: &str = "player_get_banner_id";

impl Operation for PlayerGetBannerIdOp {
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
