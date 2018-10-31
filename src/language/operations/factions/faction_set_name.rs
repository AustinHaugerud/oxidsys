use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FactionSetNameOp;

const DOC: &str =
    "Sets the name of the faction. See also (str_store_faction_name) in String Operations.";

pub const OP_CODE: u32 = 1275;

pub const IDENT: &str = "faction_set_name";

impl Operation for FactionSetNameOp {
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
                make_param_doc("<faction_id>", ""),
                make_param_doc("<string>", ""),
            ],
        }
    }
}
