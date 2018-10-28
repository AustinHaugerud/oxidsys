use language::operations::Operation;

pub struct PropInstanceGetCurrentDeformFrameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2616;

pub const IDENT: &str = "prop_instance_get_current_deform_frame";

impl Operation for PropInstanceGetCurrentDeformFrameOp {
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
