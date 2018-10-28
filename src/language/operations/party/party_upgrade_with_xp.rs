use language::operations::Operation;

pub struct PartyUpgradeWithXpOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1673;

pub const IDENT: &str = "party_upgrade_with_xp";

impl Operation for PartyUpgradeWithXpOp {
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
