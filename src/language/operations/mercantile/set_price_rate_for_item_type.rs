use language::operations::Operation;

pub struct SetPriceRateForItemTypeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1172;

pub const IDENT: &str = "set_price_rate_for_item_type";

impl Operation for SetPriceRateForItemTypeOp {
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
