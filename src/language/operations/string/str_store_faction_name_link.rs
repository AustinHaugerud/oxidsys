use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreFactionNameLinkOp;

const DOC : &str = "Stores faction name as an internal game link. Resulting string can be used in game notes, will be highlighted, and clicking on it will redirect the player to the details page of the referenced faction.";

pub const OP_CODE: u32 = 2343;

pub const IDENT: &str = "str_store_faction_name_link";

impl Operation for StrStoreFactionNameLinkOp {
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
                make_param_doc("<string_register>", ""),
                make_param_doc("<faction_id>", ""),
            ],
        }
    }
}
