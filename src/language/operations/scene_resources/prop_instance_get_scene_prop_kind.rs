use language::operations::Operation;

pub struct PropInstanceGetScenePropKindOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1853;

pub const IDENT: &str = "prop_instance_get_scene_prop_kind";

impl Operation for PropInstanceGetScenePropKindOp {
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
