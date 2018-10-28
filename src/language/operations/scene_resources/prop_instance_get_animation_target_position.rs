use language::operations::Operation;

pub struct PropInstanceGetAnimationTargetPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1863;

pub const IDENT: &str = "prop_instance_get_animation_target_position";

impl Operation for PropInstanceGetAnimationTargetPositionOp {
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
