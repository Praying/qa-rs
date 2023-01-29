from qars import qars_core
from qafactor.qautil import QA_util_get_trade_range
from pyfolio import create_returns_tear_sheet
import polars as pl
import datetime
import rqdatac as rq
import pandas as pd
import orjson
class QARS_Backtest_Local():
    def __init__(self, account, trade_gap=5):
        self.account_name = account
        
        self.trade_gap =trade_gap
    def run_backtest(self, signal):
        signal = signal.select(['date', 'order_book_id', 'weight'])\
            .filter(pl.col('weight')!=0).to_dicts()
        tradepanel = {str(t): [i for i in signal if i['date'] == t] for t in {i['date'] for i in signal}}
        insertdata = []
        self.market = qars_core.QAMarket('blex_arbit', '/opt/cache/data/mktdata.pq')
        
        self.market.init_market({self.account_name:20000000})
        idx = 0
        for i in QA_util_get_trade_range('2021-01-04', '2022-11-01'):
            idx+=1
            self.market.set_date(i)
            self.market.before_trade()
            
            if idx%self.trade_gap==0:
                
                cols = tradepanel.get(i, [])
                targets = {}
                coeff = len(cols)

                if len(cols)>0:
                    #print(cols)
                    t1 = {}
                    for slices in cols:
                        t1[slices['order_book_id']] = slices['weight']
                    #print(i, t1)
                    self.market.target_weight({self.account_name: t1}, 'next_open')
                else:
                    #print(i, {})
                    self.market.target_weight({self.account_name: {}}, 'next_open')
            else:
                pass
            settlemsg = self.market.settle()
            dailyqifi = orjson.loads(settlemsg)
            insertdata.extend(list(dailyqifi.values()))
        self.market.finish()

        self.result = insertdata
    def analysis_pref(self):
        return create_returns_tear_sheet(self.get_account().pct_change().dropna(), benchmark_rets=rq.get_price('000905.XSHG', '2021-01-04', '2022-11-01',fields= 'close').pct_change().droplevel(0).close.dropna())
    def get_account(self):
        accounts_msg = [i['accounts']['balance'] for i in self.result if i['account_cookie']==self.account_name]
        
        date_msg = [i['trading_day'] for i in self.result if i['account_cookie']==self.account_name]
        
        res =  pd.Series(accounts_msg, index=date_msg, name=self.account_name)
        res.index = pd.to_datetime(res.index)
        return res
    
    def get_trade(self):
        trades = []
        for datepanel in self.result:
            if datepanel['account_cookie']==self.account_name:
                if len(datepanel['trades'])>0:
                    trades.extend(list(datepanel['trades'].values()))
        trades = pd.DataFrame(trades) 
        trades = trades.assign(date = trades.trade_date_time.apply(
                    lambda x:  datetime.datetime.fromtimestamp(x/1000000000)))

        return trades.query('volume>0').drop(['trade_id', 'exchange_id', 'order_id', 'exchange_trade_id'], axis=1)
    def get_positions(self):
        pos_msg = [pd.DataFrame(i['positions']) for i in self.result if i['account_cookie']==self.account_name]

        date_msg = [i['trading_day'] for i in self.result if i['account_cookie']==self.account_name]

        posx = []
        for i in range(len(pos_msg)):
            pos= pos_msg[i]
            if len(pos)>0:
                pos = pos.T.assign(date=date_msg[i])
            posx.append(pos)
        return pd.concat(posx).set_index(['date', 'instrument_id'])

    def get_tradepair(self):
        #print(self.account_name)
        res= pl.DataFrame(orjson.loads(self.market.get_tradepair(self.account_name))).with_columns([
            ((pl.col('open_datetime')+28800000000000)/1000).cast(pl.Datetime),
            ((pl.col('close_datetime')+28800000000000)/1000).cast(pl.Datetime),
            (pl.col('hold_gap')/86400.0).cast(pl.Int32),
        ])
        return res.with_columns([
            pl.col('open_datetime').cast(pl.Date).alias('opendate'),
            pl.col('close_datetime').cast(pl.Date).alias('closedate')])\
            .sort('opendate')
    def get_report(self):
        pass
