use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStorePartyNameLinkOp;

const DOC : &str = "Stores party name as an internal game link. Resulting string can be used in game notes, will be highlighted, and clicking on it will redirect the player to the details page of the referenced party.";

pub const OP_CODE: u32 = 2342;

pub const IDENT: &str = "str_store_party_name_link";

impl Operation for StrStorePartyNameLinkOp {
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
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}
