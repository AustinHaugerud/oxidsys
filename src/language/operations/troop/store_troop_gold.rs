use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTroopGoldOp;

const DOC: &str = "Retrieves total number of gold that the troop has.";

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

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<troop_id>", ""),
            ],
        }
    }
}
