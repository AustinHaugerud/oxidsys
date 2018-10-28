use language::operations::Operation;

pub struct ChangeScreenExchangeWithPartyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2050;

pub const IDENT: &str = "change_screen_exchange_with_party";

impl Operation for ChangeScreenExchangeWithPartyOp {
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
