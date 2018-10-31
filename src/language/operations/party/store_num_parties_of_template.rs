use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreNumPartiesOfTemplateOp;

const DOC: &str =
    "Stores number of active parties which were created using specified party template.";

pub const OP_CODE: u32 = 2310;

pub const IDENT: &str = "store_num_parties_of_template";

impl Operation for StoreNumPartiesOfTemplateOp {
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
                make_param_doc("<party_template_id>", ""),
            ],
        }
    }
}
