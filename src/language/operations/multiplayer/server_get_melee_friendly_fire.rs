use language::operations::Operation;

pub struct ServerGetMeleeFriendlyFireOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 493;

pub const IDENT: &str = "server_get_melee_friendly_fire";

impl Operation for ServerGetMeleeFriendlyFireOp {
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
