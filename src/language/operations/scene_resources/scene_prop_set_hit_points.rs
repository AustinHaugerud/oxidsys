use language::operations::Operation;

pub struct ScenePropSetHitPointsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1814;

pub const IDENT: &str = "scene_prop_set_hit_points";

impl Operation for ScenePropSetHitPointsOp {
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
