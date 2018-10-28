use language::operations::Operation;

pub struct ItemGetFoodQualityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2716;

pub const IDENT: &str = "item_get_food_quality";

impl Operation for ItemGetFoodQualityOp {
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
