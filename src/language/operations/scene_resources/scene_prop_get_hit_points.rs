use language::operations::Operation;

pub struct ScenePropGetHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1815;

pub const IDENT: &str = "scene_prop_get_hit_points";

impl Operation for ScenePropGetHitPointsOp {
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
