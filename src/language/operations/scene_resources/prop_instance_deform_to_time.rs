use language::operations::Operation;

pub struct PropInstanceDeformToTimeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2610;

pub const IDENT: &str = "prop_instance_deform_to_time";

impl Operation for PropInstanceDeformToTimeOp {
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
