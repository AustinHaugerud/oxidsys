use language::operations::Operation;

pub struct TroopLootTroopOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1539;

pub const IDENT: &str = "troop_loot_troop";

impl Operation for TroopLootTroopOp {
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
