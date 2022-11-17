use crate::qaengine::context::*;
use crate::qamarket::marketsys::*;
use crate::qautil::tradedate::{QA_util_datelist_to_string, QATradeDate};

use polars::prelude::DataFrame;

use pyo3::prelude::*;

//=========================================================================================================
/// 回测引擎 Python Version
#[pyclass]
pub struct QABacktest {
    pub start: String,
    pub end: String
}

impl Default for QABacktest {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl QABacktest {
    #[new]
    pub fn new() -> Self {
        QABacktest {
            start: "1970-01-01".to_string(),
            end: "1970-01-01".to_string()
        }
    }

    #[allow(unused_variables)]
    pub fn run(&mut self, context: ContextWrapper, py: Python) {
        let mkt: Py<QAMarket> = context
            .py_context
            .getattr(py, "mkt")
            .unwrap()
            .extract(py)
            .unwrap();
        mkt.borrow_mut(py).init();

        //循环内不处理最后一个bar,所以range-1

        context.on_init(py);
        context.on_session_begin(py);

        let null_date = "1970-01-01";
        for date in QA_util_datelist_to_string(QATradeDate::new().get_index(&self.start, &self.end )) {
            let data= mkt.borrow_mut(py).data.get_date(&date);

            
            // 换日bar
            context.on_calc(py);
            //--------------------------------------------------------------------------
            // trade_date 结束
            context.on_session_end(py);
            // settle and change
            mkt.borrow_mut(py).settle(&data);
            // 新trade_date 开始 | 未开盘 | 此时除了trade_date已切换,其他idx都等同于on_session_end时
            context.on_session_begin(py);
            // 更新idx_today_begin
 
        }



        //回测结束事件
        context.on_finish(py);
    }
}
//=========================================================================================================
// 回测引擎 Rust Version
// #[allow(dead_code)]
// pub struct QABacktestRS {
//     pub mkt: QAMarket,
// }

// impl QABacktestRS {
//     pub fn new(account_id: Vec<&str>, strategy_id: &str, data: DataFrame, init_cash: f64) -> Self {
//         let mkt = QAMarket::new();
//         QABacktestRS { mkt }
//     }

//     #[allow(unused_variables)]
//     pub fn run(&mut self, mut context: impl Context) {
//         self.mkt.init();

//         //循环内不处理最后一个bar,所以range-1
//         let range = self.mkt.index.len() - 1;

//         context.on_init(&mut self.mkt);
//         context.on_session_begin(&mut self.mkt);

//         let null_date = NaiveDate::from_ymd(1970, 1, 1);
//         for _i in 0..range {
//             let _date = self.mkt.on_step(false);

//             if _date == null_date {
//                 // 日内连续bar
//                 context.on_calc(&mut self.mkt);
//             } else {
//                 // 换日bar
//                 context.on_calc(&mut self.mkt);
//                 //--------------------------------------------------------------------------
//                 // trade_date 结束
//                 context.on_session_end(&mut self.mkt);
//                 // settle and change
//                 self.mkt.settle(_date);
//                 // 新trade_date 开始 | 未开盘 | 此时除了trade_date已切换,其他idx都等同于on_session_end时
//                 context.on_session_begin(&mut self.mkt);
//                 // 更新idx_today_begin
//                 self.mkt.update_idx_today_begin();
//                 //--------------------------------------------------------------------------
//             }
//             self.mkt.on_step_end();
//         }
//         //==========================================================================
//         //Final bar
//         let _date = self.mkt.on_step(true);

//         context.on_calc(&mut self.mkt);

//         //trade_date 结束
//         context.on_session_end(&mut self.mkt);
//         //回测结束时清仓
//         //self.mkt.close_all("close");
//         //settle
//         self.mkt.settle(self.mkt.today);

//         //回测结束事件
//         context.on_finish(&mut self.mkt);
//     }
// }
