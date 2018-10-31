use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTroopCountPrisonersOp;

const DOC: &str =
    "Apparently deprecated, duplicates (party_get_num_prisoners). Not used in Native.";

pub const OP_CODE: u32 = 2161;

pub const IDENT: &str = "store_troop_count_prisoners";

impl Operation for StoreTroopCountPrisonersOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("[party_id]", ""),
            ],
        }
    }
}
