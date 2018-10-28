use language::operations::Operation;

pub struct ServerSetFriendlyFireDamageSelfRatioOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 496;

pub const IDENT: &str = "server_set_friendly_fire_damage_self_ratio";

impl Operation for ServerSetFriendlyFireDamageSelfRatioOp {
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
