use language::operations::Operation;

pub struct ScenePropFadeOutOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1822;

pub const IDENT: &str = "scene_prop_fade_out";

impl Operation for ScenePropFadeOutOp {
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
