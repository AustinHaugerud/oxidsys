use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRandomPartyOfTemplateOp;

const DOC : &str = "Retrieves one random party which was created using specified party template. Fails if no party exists with provided template.";

pub const OP_CODE: u32 = 2311;

pub const IDENT: &str = "store_random_party_of_template";

impl Operation for StoreRandomPartyOfTemplateOp {
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
