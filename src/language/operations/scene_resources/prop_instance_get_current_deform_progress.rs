use language::operations::Operation;

pub struct PropInstanceGetCurrentDeformProgressOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2615;

pub const IDENT: &str = "prop_instance_get_current_deform_progress";

impl Operation for PropInstanceGetCurrentDeformProgressOp {
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
