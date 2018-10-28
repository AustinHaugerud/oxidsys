use language::operations::Operation;

pub struct ServerSetMeleeFriendlyFireOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 494;

pub const IDENT: &str = "server_set_melee_friendly_fire";

impl Operation for ServerSetMeleeFriendlyFireOp {
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
