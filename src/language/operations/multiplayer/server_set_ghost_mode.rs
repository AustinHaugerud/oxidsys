use language::operations::Operation;

pub struct ServerSetGhostModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 488;

pub const IDENT: &str = "server_set_ghost_mode";

impl Operation for ServerSetGhostModeOp {
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
