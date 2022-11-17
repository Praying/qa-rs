use crate::qamarket::marketsys::QAMarket;
use pyo3::prelude::*;

//======================================================================================================
/// Python Context Wrapper
#[pyclass]
#[derive(Debug, Clone)]
pub struct ContextWrapper {
    pub py_context: Py<PyAny>,
}
#[pymethods]
impl ContextWrapper {
    #[new]
    pub fn new(py_context: Py<PyAny>) -> Self {
        ContextWrapper { py_context }
    }
}
/// 用于调用py_context的对应方法 不向Python暴露
impl ContextWrapper {
    pub fn on_init(&self, py: Python) {
        self.py_context.as_ref(py).call_method0("on_init").unwrap();
    }

    pub fn on_session_begin(&self, py: Python) {
        self.py_context
            .as_ref(py)
            .call_method0("on_session_begin")
            .unwrap();
    }

    pub fn on_session_end(&self, py: Python) {
        self.py_context
            .as_ref(py)
            .call_method0("on_session_end")
            .unwrap();
    }

    pub fn on_calc(&self, py: Python) {
        self.py_context.as_ref(py).call_method0("on_calc").unwrap();
    }

    pub fn on_finish(&self, py: Python) {
        self.py_context
            .as_ref(py)
            .call_method0("on_finish")
            .unwrap();
        //let args = (acc, datacube);
    }
}

//======================================================================================================
/// Rust策略-基础方法
#[allow(unused_variables)]
pub trait Context {
    /// 策略运行前初始化
    fn on_init(&mut self, datacube: &mut QAMarket) {}

    /// 单个code行情事件
    fn on_bar(&mut self) {}

    /// 每日盘前事件 before market open,注意此时trade_date已切到新交易日但无法获取当日数据,可用于提取timeline
    fn on_session_begin(&mut self, datacube: &mut QAMarket) {}

    /// 每日盘尾或盘后事件,on or after market end,与on_session_begin共享running_time
    fn on_session_end(&mut self, datacube: &mut QAMarket) {}

    /// 核心逻辑,同一时间戳全部bar结束方可触发
    fn on_calc(&mut self, datacube: &mut QAMarket) {}

    /// 每日盘后事件,注意此时已完成清仓和结算,回测全部结束
    fn on_finish(&mut self, datacube: &mut QAMarket) {}

    /// 换月平仓
    fn clear_change(&mut self) {
        //TODO:
    }

    /// 换月开仓
    fn restore_change(&mut self) {
        //TODO:
    }
}
//======================================================================================================
