use language::operations::Operation;

pub struct StoreCurrentSceneOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2211;

pub const IDENT: &str = "store_current_scene";

impl Operation for StoreCurrentSceneOp {
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
