# data for backtest


data 以parquet - > polars - > hashmap 双层链表的形式实现

```json
{"date": {"stock": Kline, "stock": Kline}}
```

pub struct Kline {
    pub order_book_id: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub limit_up: f64,
    pub limit_down: f64,
    pub total_turnover: f64,
    pub split_coefficient_to: f64,
    pub dividend_cash_before_tax: f64,
}

Kline 以不复权的原始数据存储, 支持股票/期货同时存在于一个截面中, 使用


```python
import polars as pl

data1 = pl.read_parquet("data/bfq_*", use_pyarrow=True, memory_map=True)
data2 = pl.read_parquet("xdxr.pq")
data1.join(data2, on=['order_book_id', 'date'], how='left').write_parquet("mkt.pq")
```
用于生成最新市场截面


