use language::operations::Operation;

pub struct ServerGetFriendlyFireDamageFriendRatioOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 497;

pub const IDENT: &str = "server_get_friendly_fire_damage_friend_ratio";

impl Operation for ServerGetFriendlyFireDamageFriendRatioOp {
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
