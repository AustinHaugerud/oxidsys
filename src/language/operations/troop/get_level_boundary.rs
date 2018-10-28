use language::operations::Operation;

pub struct GetLevelBoundaryOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 991;

pub const IDENT: &str = "get_level_boundary";

impl Operation for GetLevelBoundaryOp {
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
