use language::operations::Operation;

pub struct ReplacePropInstanceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1889;

pub const IDENT: &str = "replace_prop_instance";

impl Operation for ReplacePropInstanceOp {
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
