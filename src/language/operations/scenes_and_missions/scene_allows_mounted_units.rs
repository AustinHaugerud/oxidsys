use language::operations::Operation;

pub struct SceneAllowsMountedUnitsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1834;

pub const IDENT: &str = "scene_allows_mounted_units";

impl Operation for SceneAllowsMountedUnitsOp {
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
