use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddGoldAsXpOp;

const DOC : &str = "Adds a certain amount of experience points, depending on the amount of gold specified. Conversion rate is unclear and apparently somewhat randomized (three runs with 1000 gold produced values 1091, 804 and 799).";

pub const OP_CODE: u32 = 1063;

pub const IDENT: &str = "add_gold_as_xp";

impl Operation for AddGoldAsXpOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 1,
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<value>", ""),
                make_param_doc("[troop_id]", ""),
            ],
        }
    }
}
