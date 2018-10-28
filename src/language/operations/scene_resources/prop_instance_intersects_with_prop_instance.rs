use language::operations::Operation;

pub struct PropInstanceIntersectsWithPropInstanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1880;

pub const IDENT: &str = "prop_instance_intersects_with_prop_instance";

impl Operation for PropInstanceIntersectsWithPropInstanceOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}
