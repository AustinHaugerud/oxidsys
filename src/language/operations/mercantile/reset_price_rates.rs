use language::operations::Operation;

pub struct ResetPriceRatesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1170;

pub const IDENT: &str = "reset_price_rates";

impl Operation for ResetPriceRatesOp {
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
