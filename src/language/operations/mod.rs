mod flow_control;

/**
 * A special thank you to Alexander Lomski AKA Lav for providing an excellent basis
 * on how to organize and document operations.
*/

pub trait Operation {
    fn op_code(&self) -> u16;
    fn documentation(&self) -> &str;
}
