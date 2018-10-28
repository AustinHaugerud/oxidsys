use language::operations::Operation;

pub struct PropInstanceClearAttachedMissilesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1885;

pub const IDENT: &str = "prop_instance_clear_attached_missiles";

impl Operation for PropInstanceClearAttachedMissilesOp {
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
