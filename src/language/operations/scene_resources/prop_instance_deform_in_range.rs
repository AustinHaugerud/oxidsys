use language::operations::Operation;

pub struct PropInstanceDeformInRangeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2611;

pub const IDENT: &str = "prop_instance_deform_in_range";

impl Operation for PropInstanceDeformInRangeOp {
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
