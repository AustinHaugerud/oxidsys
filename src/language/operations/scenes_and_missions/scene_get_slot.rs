use language::operations::Operation;

pub struct SceneGetSlotOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 523;

pub const IDENT: &str = "scene_get_slot";

impl Operation for SceneGetSlotOp {
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
