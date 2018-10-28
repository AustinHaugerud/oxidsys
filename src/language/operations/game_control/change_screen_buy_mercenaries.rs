use language::operations::Operation;

pub struct ChangeScreenBuyMercenariesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2045;

pub const IDENT: &str = "change_screen_buy_mercenaries";

impl Operation for ChangeScreenBuyMercenariesOp {
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
