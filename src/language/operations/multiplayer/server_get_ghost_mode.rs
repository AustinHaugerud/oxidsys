use language::operations::Operation;

pub struct ServerGetGhostModeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 487;

pub const IDENT: &str = "server_get_ghost_mode";

impl Operation for ServerGetGhostModeOp {
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
