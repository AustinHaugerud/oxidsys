use language::operations::Operation;

pub struct SetBackgroundMeshOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2031;

pub const IDENT: &str = "set_background_mesh";

impl Operation for SetBackgroundMeshOp {
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
