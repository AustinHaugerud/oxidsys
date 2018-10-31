use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateProgressOverlayOp;

const DOC : &str = "Creates progress bar overlay, with positions of the bar varying between min and max values. Current value of the progress bar can be changed with (overlay_set_val). Returns bar's overlay_id.";

pub const OP_CODE: u32 = 915;

pub const IDENT: &str = "create_progress_overlay";

impl Operation for CreateProgressOverlayOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<min_value>", ""),
                make_param_doc("<max_value>", ""),
            ],
        }
    }
}
