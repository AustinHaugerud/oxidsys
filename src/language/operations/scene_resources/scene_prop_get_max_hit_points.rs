use language::operations::Operation;

pub struct ScenePropGetMaxHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1816;

pub const IDENT: &str = "scene_prop_get_max_hit_points";

impl Operation for ScenePropGetMaxHitPointsOp {
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
