use language::operations::Operation;

pub struct ItemGetHorseScaleOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2713;

pub const IDENT: &str = "item_get_horse_scale";

impl Operation for ItemGetHorseScaleOp {
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
