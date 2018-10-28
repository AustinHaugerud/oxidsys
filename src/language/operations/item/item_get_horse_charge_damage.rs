use language::operations::Operation;

pub struct ItemGetHorseChargeDamageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2722;

pub const IDENT: &str = "item_get_horse_charge_damage";

impl Operation for ItemGetHorseChargeDamageOp {
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
