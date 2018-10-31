use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetVisitorOp;

const DOC : &str = "Adds the specified troop as the visitor to the entry point of the scene defined with (modify_visitors_at_site). Entry point must have mtef_visitor_source type. Optional DNA parameter allows for randomization of agent looks (only applies to non-hero troops).";

pub const OP_CODE: u32 = 1263;

pub const IDENT: &str = "set_visitor";

impl Operation for SetVisitorOp {
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
                make_param_doc("<entry_no>", ""),
                make_param_doc("<troop_id>", ""),
                make_param_doc("[<dna>]", ""),
            ],
        }
    }
}
