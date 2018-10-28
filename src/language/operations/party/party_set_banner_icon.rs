use language::operations::Operation;

pub struct PartySetBannerIconOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1677;

pub const IDENT: &str = "party_set_banner_icon";

impl Operation for PartySetBannerIconOp {
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
