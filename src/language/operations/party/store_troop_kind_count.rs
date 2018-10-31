use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTroopKindCountOp;

const DOC : &str = "Counts number of troops of specified type in player's party. Deprecated, use party_count_members_of_type instead.";

pub const OP_CODE: u32 = 2158;

pub const IDENT: &str = "store_troop_kind_count";

impl Operation for StoreTroopKindCountOp {
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
                make_param_doc("<troop_type_id>", ""),
            ],
        }
    }
}
