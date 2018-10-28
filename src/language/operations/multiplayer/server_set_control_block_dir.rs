use language::operations::Operation;

pub struct ServerSetControlBlockDirOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 483;

pub const IDENT: &str = "server_set_control_block_dir";

impl Operation for ServerSetControlBlockDirOp {
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
