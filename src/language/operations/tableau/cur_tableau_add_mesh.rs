use language::operations::Operation;

pub struct CurTableauAddMeshOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1992;

pub const IDENT: &str = "cur_tableau_add_mesh";

impl Operation for CurTableauAddMeshOp {
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
