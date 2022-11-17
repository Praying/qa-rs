use crate::qadata::marketcenter::Kline;
use crate::qamarket::marketsys::QAMarket;
use hashbrown::HashMap;
use pyo3::prelude::*;

//======================================================================================================
/// Python Strategy Wrapper
#[pyclass]
#[derive(Debug, Clone)]
pub struct StrategyWrapper {
    pub py_strategy: Py<PyAny>,

}
#[pymethods]
impl StrategyWrapper {
    #[new]
    pub fn new(py_strategy: Py<PyAny>) -> Self {
        StrategyWrapper {
            py_strategy,
        }
    }
}
/// 用于调用py_strategy的对应方法 不向Python暴露
impl StrategyWrapper {
    pub fn on_init(&self, py: Python) {
        self.py_strategy.as_ref(py).call_method0("on_init").unwrap();
    }

    pub fn on_session_begin(&self, py: Python) {
        self.py_strategy
            .as_ref(py)
            .call_method0("on_session_begin")
            .unwrap();
    }

    pub fn on_session_end(&self, py: Python) {
        self.py_strategy
            .as_ref(py)
            .call_method0("on_session_end")
            .unwrap();
    }



    pub fn on_calc(&self, py: Python) {
        self.py_strategy.as_ref(py).call_method0("on_calc").unwrap();
    }

    pub fn on_finish(&self, py: Python) {
        self.py_strategy
            .as_ref(py)
            .call_method0("on_finish")
            .unwrap();
        //let args = (acc, data);
    }
}

//======================================================================================================
/// Rust策略-基础方法
#[allow(unused_variables)]
pub trait Strategy {
    /// 策略运行前初始化
    fn on_init(&mut self) {}

    /// 单个code行情事件
    fn on_bar(&mut self) {}

    /// 每日盘前事件 before market open,注意此时trade_date已切到新交易日但无法获取当日数据,可用于提取timeline
    fn on_session_begin(&mut self, data: &HashMap<String, Kline>) {}

    /// 每日盘尾或盘后事件,on or after market end,与on_session_begin共享running_time
    fn on_session_end(&mut self, data: &HashMap<String, Kline>) {}

    /// 核心逻辑,同一时间戳全部bar结束方可触发
    fn on_calc(&mut self, data: &HashMap<String, Kline>) {}

    fn on_calc_hp(&mut self) {
        // this is for high performance
        // reduce all
    }
    /// 每日盘后事件,注意此时已完成清仓和结算,回测全部结束
    fn on_finish(&mut self, data: &HashMap<String, Kline>) {}
}
//======================================================================================================
