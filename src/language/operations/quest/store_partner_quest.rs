use language::operations::Operation;

pub struct StorePartnerQuestOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2240;

pub const IDENT: &str = "store_partner_quest";

impl Operation for StorePartnerQuestOp {
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
