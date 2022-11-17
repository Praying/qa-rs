import qars


class demo_strategy(qars.Strategy):
    def init(self):
        pass

    def on_calc(self):
        # self.market
        pass


def run_market_test():
    market = qars.core.QAMarket('C:\\quant\\data\\mkt.pq')
    market.init_market(['x1', 'x2'])
    market.set_date('2022-10-21')

    market.target_amount({
        "x1": {"000002.XSHE": 500, "600010.XSHG": 500},
        "x2": {"000002.XSHE": 500, "600010.XSHG": 500}
    })
    print(market.get_account_qifi('x1'))


if __name__ == "__main__":
    # backtest = qars.Backtest('2022-01-01', '2022-01-21', 'C:\\quant\\data\\mkt.pq')
    # backtest.run(demo_strategy( 'C:\\quant\\data\\mkt.pq'))

    run_market_test()
