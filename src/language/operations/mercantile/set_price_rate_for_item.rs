use language::operations::Operation;

pub struct SetPriceRateForItemOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1171;

pub const IDENT: &str = "set_price_rate_for_item";

impl Operation for SetPriceRateForItemOp {
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
