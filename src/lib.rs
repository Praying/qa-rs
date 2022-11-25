pub mod qaaccount;
pub mod qaconnector;
pub mod qadata;
pub mod qaengine;
pub mod qaenv;
pub mod qalog;
pub mod qamarket;
pub mod qaprotocol;
pub mod qautil;
pub mod qaservice;
use mimalloc::MiMalloc;
use pyo3::prelude::*;
use qaaccount::account::QA_Account;
use qaengine::{backtest::QABacktest, strategy::StrategyWrapper};
use qamarket::marketsys::QAMarket;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
//======================================================================================================
/// VARLAR_CORE
#[pymodule]
fn qars_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<QA_Account>()?;
    m.add_class::<QABacktest>()?;
    m.add_class::<QAMarket>()?;
    m.add_class::<StrategyWrapper>()?;
    Ok(())
}
