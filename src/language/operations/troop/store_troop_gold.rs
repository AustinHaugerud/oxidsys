use language::operations::Operation;

pub struct StoreTroopGoldOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2149;

pub const IDENT: &str = "store_troop_gold";

impl Operation for StoreTroopGoldOp {
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
