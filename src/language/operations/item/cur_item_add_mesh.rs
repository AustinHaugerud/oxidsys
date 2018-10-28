use language::operations::Operation;

pub struct CurItemAddMeshOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1964;

pub const IDENT: &str = "cur_item_add_mesh";

impl Operation for CurItemAddMeshOp {
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
