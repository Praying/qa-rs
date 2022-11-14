pub mod qaaccount;
pub mod qaprotocol;
pub mod qaenv;
pub mod qalog;
pub mod qautil;
pub mod qadata;
pub mod qamarket;
pub mod qaconnector;



use pyo3::prelude::*;
use qaaccount::account::QA_Account;
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
//======================================================================================================
/// VARLAR_CORE
#[pymodule]
fn qars_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QA_Account>()?;
    Ok(())
}