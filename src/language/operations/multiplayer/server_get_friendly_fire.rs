use language::operations::Operation;

pub struct ServerGetFriendlyFireOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 480;

pub const IDENT: &str = "server_get_friendly_fire";

impl Operation for ServerGetFriendlyFireOp {
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
