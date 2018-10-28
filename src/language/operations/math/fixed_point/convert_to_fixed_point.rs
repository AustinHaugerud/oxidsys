use language::operations::Operation;

pub struct ConvertToFixedPointOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2130;

pub const IDENT: &str = "convert_to_fixed_point";

impl Operation for ConvertToFixedPointOp {
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
