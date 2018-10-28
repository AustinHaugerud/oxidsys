use language::operations::Operation;

pub struct TroopGetUpgradeTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1561;

pub const IDENT: &str = "troop_get_upgrade_troop";

impl Operation for TroopGetUpgradeTroopOp {
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
