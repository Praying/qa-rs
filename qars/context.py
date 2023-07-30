
from . import qars_core as core
import datetime as dt
from typing import Union

import polars as pl
import pandas as pd
import sys
sys.path.append('./')


class Strategy:
    def __init__(
            self,
            data_path: str,

    ):

        self.market = core.QAMarket(data_path)
        self.hold_period = 0

    # --------------------------------------------------------------------------------------------

    def register_account(self, account_name: str, init_cash: float, ):
        self.market.register_account(account_name, init_cash)

    @property
    def datetime(self) -> str:
        return self.market.today

    def init_cash(self) -> float:
        """获取init_cash"""
        return self.cube.init_cash

    def pre_balance(self) -> float:
        """获取pre_balance"""
        return self.cube.pre_balance

    @property
    def pre_float_profit(self) -> float:
        """获取pre_float_profit"""
        return self.cube.pre_float_profit

    @property
    def close_profit(self) -> float:
        """获取close_profit"""
        return self.cube.close_profit

    @property
    def commission(self) -> float:
        """获取commission"""
        return self.cube.commission

    @property
    def margin(self) -> float:
        """获取margin"""
        return self.cube.margin

    @property
    def is_open_today(self) -> bool:
        return self.cube.is_open_today

    @property
    def env(self):
        return self.cube.env

    # --------------------------------------------------------------------------------------------
    def on_init(self):
        """
        策略运行前初始化
        """
        print('py on_init')
        self.market.init_market({'000001': 2000000, '000002': 5000000})

    def on_bar(self):
        """
        单个code行情事件
        """
        print('py on_bar')

    def on_session_begin(self):
        """
        每日盘前事件 before market open,注意此时trade_date已切到新交易日但无法获取当日数据,可用于提取timeline
        """
        print('py on_session_begin')

    def on_session_end(self):
        """
        每日盘尾或盘后事件,on or after market end,与on_session_begin共享running_time
        """
        print('py on_session_end')

    def on_calc(self):
        """
        核心逻辑,同一时间戳全部bar结束方可触发
        """
        print('py on_calc')

    def on_finish(self):
        """
        每日盘后事件,注意此时已完成清仓和结算,回测全部结束
        """
        print('py on_finish')

    # --------------------------------------------------------------------------------------------

    def close_all(self, label: str):
        """全部平仓"""
        self.cube.close_all(label)

    def clear_timeline(self, price_label: str):
        """清理当日timeline记录"""
        self.cube.clear_timeline(price_label)

    def target(self, panel: dict, label: str = 'close'):
        """基于品种权重调仓 target_panel:品种->手数带正负 label:open|high|low|close"""
        self.cube.target(panel, label)

    def target_byorder(self, panel: dict, label: str = 'close'):
        """基于品种权重调仓 target_panel:品种->手数带正负 label:open|high|low|close

        target_panel 和target的区别是targetpanel采用了先平仓再开仓的模式
        """
        self.cube.target_byorder(panel, label)

    def buy_open(self, code: str, price: float, amount: int, timeline: bool = False):
        """timeline -> bool 是否记录timeline"""
        if amount >= 1:
            amount = round(amount, 0)
            self.cube.buy_open(code, price, amount)
            self.cube.create_timeline(
                code, price, amount, "long", self.hold_period
            ) if timeline else None

    def sell_open(self, code: str, price: float, amount: int, timeline: bool = False):
        """timeline -> bool 是否记录timeline"""
        if amount >= 1:
            amount = round(amount, 0)
            self.cube.sell_open(code, price, amount)
            self.cube.create_timeline(
                code, price, amount, "short", self.hold_period
            ) if timeline else None

    def buy_close(self, code: str, price: float, amount: int):
        if amount >= 1:
            amount = round(amount, 0)
            self.cube.buy_close(code, price, amount)

    def sell_close(self, code: str, price: float, amount: int):
        if amount >= 1:
            amount = round(amount, 0)
            self.cube.sell_close(code, price, amount)

    def buy_closetoday(self, code: str, price: float, amount: int):
        if amount >= 1:
            amount = round(amount, 0)
            self.cube.buy_closetoday(code, price, amount)

    def sell_closetoday(self, code: str, price: float, amount: int):
        if amount >= 1:
            amount = round(amount, 0)
            self.cube.sell_closetoday(code, price, amount)

    # --------------------------------------------------------------------------------------------
    def get_float_profit(self, label: str = "close") -> float:
        """获取float_profit | 需要update_account"""
        self.cube.update_account(label)
        return self.cube.float_profit

    def get_balance(self, label: str) -> float:
        """获取balance | 需要update_account"""
        self.cube.update_account(label)
        return self.cube.balance

    def get_available(self, label: str) -> float:
        """获取available  | 需要update_account"""
        self.cube.update_account(label)
        return self.cube.available

    def get_map(self, key: str = "code", value: str = "close", shift: int = 0) -> dict:
        """
        获取bar对应的dict,通过shift决定偏移量,shift=-1时获得下一个bar的dict,按bar顺序排序
        目前仅支持f64 column
        """
        return self.cube.get_map(key, value, shift)

    def get_bmap(self, label: str = "close", shift: int = 0) -> dict:
        """
        获取bar对应的BTreeMap,通过shift决定偏移量,shift=-1时获得下一个bar的BTreeMap,按key排序
        目前仅支持f64 column
        """
        return self.cube.get_bmap(label, shift)

    def get_longshort_list(self, head: Union[int, float] = 0, shift: int = 0) -> list:
        """
        按当前bar的顺序获得code列表, head=0时取bar全部code,head大于1小于-1时按照整数取固定数量code,否则按照quantile百分比取code
        通过shift决定偏移量,shift=-1时获得下一个bar的code列表
        """
        return self.cube.get_longshort_list(head, shift)

    # def get_bar(self, shift: int) :
    #     """
    #     获取1个bar | 通过shift决定偏移量
    #     """
    #     offset, length = self.cube.get_bar_idx(shift)
    #     return self.data.slice(offset, length)

    # def get_n_bars(self, n: int, shift: int) -> Union[pl.DataFrame, va.DataFrame]:
    #     """
    #     获取过去N个bar | 通过shift决定偏移量
    #     """
    #     offset, length = self.cube.get_n_bars_idx(n, shift)
    #     return self.data.slice(offset, length)

    # def get_today_bars(self, shift: int) -> Union[pl.DataFrame, va.DataFrame]:
    #     """
    #     获取当前trade_date的所有bar,shift=1时不含当前bar,未支持shift>1的场景
    #     """
    #     offset, length = self.cube.get_today_bars_idx(shift)
    #     return self.data.slice(offset, length)

    def get_holding_list(self, direction: str = "all") -> list:
        """
        获取账户中的持仓品种列表 direction: sub | long | short | all
        """
        return self.cube.get_holding_list(direction)

    def get_pos_info_single(self, code: str) -> dict:
        """
        获取单个code的持仓信息
        日终结算时会为每个pos更新当日收盘价,float_profit需要更新last_price
        """
        return self.cube.get_pos_info_single(code)

    def get_pos_info(self, direction: str = "all") -> dict:
        """
        获取多个code的持仓信息 direction: sub | long | short | all
        日终结算时会为每个pos更新当日收盘价,float_profit需要更新last_price
        """
        return self.cube.get_pos_info(direction)

    def get_pos_amount(self, direction: str = "all") -> dict:
        """
        获取多个code的净持仓数量信息 direction: long | short | all
        多头持仓返回正数,空头持仓返回负数
        """
        return self.cube.get_pos_amount(direction)

    def get_volume_short(self, code: str) -> int:
        """
        获取空头持仓 | 该函数不涉及价格更新,涉及一次加法计算
        """
        return self.cube.get_volume_short(code)

    def get_volume_long(self, code: str) -> int:
        """
        获取多头持仓 | 该函数不涉及价格更新,涉及一次加法计算
        """
        return self.cube.get_volume_long(code)

    def get_open_price_long(self, code: str) -> float:
        """
        取得多头持仓成本价 | 该函数不涉及价格更新和计算,直接获取数据
        """
        return self.cube.get_open_price_long(code)

    def get_open_price_short(self, code: str) -> float:
        """
        取得空头持仓成本价 | 该函数不涉及价格更新和计算,直接获取数据
        """
        return self.cube.get_open_price_short(code)

    def update_price(self, code: str, price: Union[int, float]):
        """
        用户发起-刷新单个code价格,用于之后计算单个pos的pnl
        """
        self.cube.update_price(code, price)

    def update_price_all(self, label: str):
        """
        用户发起-刷新全部pos价格,可随时调用
        """
        self.cube.update_price_all(label)

    def update_account(self, label: str):
        """
        用户发起-该函数会自动执行update_price_all,然后基于pos存储的最新价更新acc的float_profit | balance | available, 可随时调用
        """
        self.cube.update_account(label)

    # --------------------------------------------------------------------------------------------
    def margin_cost(self, code: str, price: float, direction: str = "long") -> float:
        """
        获取一手合约占用的保证金
        """
        return self.cube.margin_cost(code, price, direction)

    def get_asset_history(self, output: str = "polars") -> Union[pl.DataFrame, pd.DataFrame]:
        """
        获取资产历史记录
        """
        history = self.cube.get_asset_history()
        history = pl.DataFrame._from_pydf(
            va.PyDataFrame.from_arrow_record_batches(history)
        )
        return (
            history
            if output == "polars"
            else history.to_pandas()
            if output == "pandas"
            else None
        )

    def get_trade_history(self, output: str = "polars") -> Union[pl.DataFrame, pd.DataFrame]:
        """
        获取交易历史记录
        """
        history = self.cube.get_trade_history()
        history = pl.DataFrame._from_pydf(
            va.PyDataFrame.from_arrow_record_batches(history)
        )
        return (
            history
            if output == "polars"
            else history.to_pandas()
            if output == "pandas"
            else None
        )

    def get_hold_history(self, output: str = "polars") -> Union[pl.DataFrame, pd.DataFrame]:
        """
        获取交易历史记录
        """
        history = self.cube.get_hold_history()
        history = pl.DataFrame._from_pydf(
            va.PyDataFrame.from_arrow_record_batches(history)
        )
        return (
            history
            if output == "polars"
            else history.to_pandas()
            if output == "pandas"
            else None
        )

    def get_hold_pivot(self, output: str = "polars") -> Union[pl.DataFrame, pd.DataFrame]:
        """
        获取交易历史记录pivot
        """
        history = self.get_hold_history().with_column(
            (pl.col('volume_long') - pl.col('volume_short')).alias('volume')).pivot('volume', 'trade_date', 'code')
        return (
            history
            if output == "polars"
            else history.to_pandas()
            if output == "pandas"
            else None
        )

    def get_holdprofit_pivot(self, output: str = "polars") -> Union[pl.DataFrame, pd.DataFrame]:
        """
        获取品种持仓盈亏当日累计值
        """
        history = self.get_hold_history().with_column(
            (pl.col('profit_long') + pl.col('profit_short')).alias('profit')).pivot('profit', 'trade_date', 'code')
        return (
            history
            if output == "polars"
            else history.to_pandas()
            if output == "pandas"
            else None
        )

    def get_holdprofit_daily(self, output: str = "polars") -> Union[pl.DataFrame, pd.DataFrame]:
        """
        获取品种持仓盈亏当日变化值
        """
        tx = self.get_holdprofit_pivot()
        history = pl.concat(
            [tx.slice(0, 1), tx.with_columns(pl.col('*').exclude('trade_date').fill_null(0).diff()).slice(1, len(tx))])
        return (
            history
            if output == "polars"
            else history.to_pandas()
            if output == "pandas"
            else None
        )

    def display_performance(self, mode="full", render_html=False):
        return va.display_performance(
            self.get_asset_history(output="pandas"),
            self.get_trade_history(output="pandas"),
            self.cube.init_cash,
            mode,
            render_html=render_html
        )

    def stop(self):
        print("SYSTEM TERMINATED")
        return va.sys.exit(0)
