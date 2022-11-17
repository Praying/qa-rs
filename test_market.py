import qars


class demo_strategy(qars.Strategy):
    def init(self):
        pass
    
    def on_calc(self):
        #self.market
        pass

def run_market_test():
    market = qars.core.QAMarket('C:\\quant\\data\\mkt.pq')
    market.init_market(['x1', 'x2'])
    market.set_date('2022-10-21')
    market.target_weight('x1', {"000001.XSHE": 0.5, "000002.XSHE": 0.5})
    market.target_weight('x2', {"000002.XSHE": 0.5, "600010.XSHG": 0.5})
    




if __name__ == "__main__":
    # backtest = qars.Backtest('2022-01-01', '2022-01-21', 'C:\\quant\\data\\mkt.pq')
    # backtest.run(demo_strategy( 'C:\\quant\\data\\mkt.pq'))

    run_market_test()