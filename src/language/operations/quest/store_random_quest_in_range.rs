use language::operations::Operation;

pub struct StoreRandomQuestInRangeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2250;

pub const IDENT: &str = "store_random_quest_in_range";

impl Operation for StoreRandomQuestInRangeOp {
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
