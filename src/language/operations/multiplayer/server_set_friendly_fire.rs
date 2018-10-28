use language::operations::Operation;

pub struct ServerSetFriendlyFireOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 481;

pub const IDENT: &str = "server_set_friendly_fire";

impl Operation for ServerSetFriendlyFireOp {
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
