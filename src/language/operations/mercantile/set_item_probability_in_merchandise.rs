use language::operations::Operation;

pub struct SetItemProbabilityInMerchandiseOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1493;

pub const IDENT: &str = "set_item_probability_in_merchandise";

impl Operation for SetItemProbabilityInMerchandiseOp {
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
