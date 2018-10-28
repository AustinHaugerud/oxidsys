use language::operations::Operation;

pub struct PropInstanceGetScaleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1852;

pub const IDENT: &str = "prop_instance_get_scale";

impl Operation for PropInstanceGetScaleOp {
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
