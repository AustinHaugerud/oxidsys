use language::operations::Operation;

pub struct ServerSetFriendlyFireDamageFriendRatioOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 498;

pub const IDENT: &str = "server_set_friendly_fire_damage_friend_ratio";

impl Operation for ServerSetFriendlyFireDamageFriendRatioOp {
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
