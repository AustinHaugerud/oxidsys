use language::operations::Operation;

pub struct ResetItemProbabilitiesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1492;

pub const IDENT: &str = "reset_item_probabilities";

impl Operation for ResetItemProbabilitiesOp {
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
