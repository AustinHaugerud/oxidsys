use language::operations::Operation;

pub struct PropInstanceSetMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2617;

pub const IDENT: &str = "prop_instance_set_material";

impl Operation for PropInstanceSetMaterialOp {
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
