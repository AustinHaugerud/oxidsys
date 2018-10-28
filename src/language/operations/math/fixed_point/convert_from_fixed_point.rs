use language::operations::Operation;

pub struct ConvertFromFixedPointOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2131;

pub const IDENT: &str = "convert_from_fixed_point";

impl Operation for ConvertFromFixedPointOp {
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
