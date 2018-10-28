use language::operations::Operation;

pub struct ServerGetControlBlockDirOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 482;

pub const IDENT: &str = "server_get_control_block_dir";

impl Operation for ServerGetControlBlockDirOp {
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
