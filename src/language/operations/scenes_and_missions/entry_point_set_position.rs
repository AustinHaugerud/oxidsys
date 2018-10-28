use language::operations::Operation;

pub struct EntryPointSetPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1781;

pub const IDENT: &str = "entry_point_set_position";

impl Operation for EntryPointSetPositionOp {
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
