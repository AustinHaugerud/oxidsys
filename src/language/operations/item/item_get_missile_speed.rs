use language::operations::Operation;

pub struct ItemGetMissileSpeedOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2709;

pub const IDENT: &str = "item_get_missile_speed";

impl Operation for ItemGetMissileSpeedOp {
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
