use language::operations::Operation;

pub struct ItemGetWeaponLengthOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2707;

pub const IDENT: &str = "item_get_weapon_length";

impl Operation for ItemGetWeaponLengthOp {
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
