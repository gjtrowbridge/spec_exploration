use ssz_types::{VariableList};
// TODO: Replace this with the actual max # of bytes an EE can have
// Currently this is arbitrarily set to 256KB max size
use typenum::U262144;
pub struct ExecutionEnvironment {
    wasm_code: VariableList<u8, U262144>,
}
