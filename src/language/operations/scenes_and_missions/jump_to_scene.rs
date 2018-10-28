use language::operations::Operation;

pub struct JumpToSceneOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1910;

pub const IDENT: &str = "jump_to_scene";

impl Operation for JumpToSceneOp {
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
