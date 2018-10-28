use language::operations::Operation;

pub struct ServerGetFriendlyFireDamageSelfRatioOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 495;

pub const IDENT: &str = "server_get_friendly_fire_damage_self_ratio";

impl Operation for ServerGetFriendlyFireDamageSelfRatioOp {
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
