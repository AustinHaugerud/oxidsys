use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreTroopNameByCountOp;

const DOC: &str =
    "Stores singular or plural troop name with number of troops (\"29 Archers\", \"1 Bandit\").";

pub const OP_CODE: u32 = 2324;

pub const IDENT: &str = "str_store_troop_name_by_count";

impl Operation for StrStoreTroopNameByCountOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<string_register>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("<number>", ""),
            ],
        }
    }
}
