use language::operations::Operation;

pub struct PropInstanceGetVariationIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1840;

pub const IDENT: &str = "prop_instance_get_variation_id";

impl Operation for PropInstanceGetVariationIdOp {
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
