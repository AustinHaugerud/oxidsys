use language::operations::Operation;

pub struct ItemGetThrustDamageTypeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2719;

pub const IDENT: &str = "item_get_thrust_damage_type";

impl Operation for ItemGetThrustDamageTypeOp {
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
