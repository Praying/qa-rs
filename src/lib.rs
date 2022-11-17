pub mod qaaccount;
pub mod qaconnector;
pub mod qadata;
pub mod qaenv;
pub mod qalog;
pub mod qamarket;
pub mod qaprotocol;
pub mod qautil;
pub mod qaengine;

use mimalloc::MiMalloc;
use pyo3::prelude::*;
use qaaccount::account::QA_Account;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
//======================================================================================================
/// VARLAR_CORE
#[pymodule]
fn qars_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QA_Account>()?;
    Ok(())
}
