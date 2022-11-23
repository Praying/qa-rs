import qars
import pymongo
import orjson
class demo_strategy(qars.Strategy):
    def init(self):
        pass

    def on_calc(self):
        # self.market
        print(self.market.to)


def run_market_test():
    market = qars.core.QAMarket('mytestportfolio', 'C:\\quant\\data\\mkt.pq')
    market.init_market({'000001': 2000000, '000002': 5000000})
    insertdata = []
    market.set_date('2022-07-20')
    market.before_trade()
    market.target_weight({
        "000001": {"000001.XSHE": 0.4, "600010.XSHG": 0.6},
        "000002": {"000001.XSHE": 0.4, "600010.XSHG": 0.6}
    }, 'next_open')
    # print(market.get_account_qifi('x1'))
    # print(market.get_portfolio())
    settlemsg = market.settle()
    dailyqifi = orjson.loads(settlemsg)
    
    insertdata.extend(list(dailyqifi.values()))

    market.set_date('2022-07-21')
    market.before_trade()

    market.target_weight({
        "000001": {"000001.XSHE": 0.4, "600010.XSHG": 0.6},
        "000002": {"000001.XSHE": 0.4, "600010.XSHG": 0.6}
    }, 'next_open')
    # print(market.get_account_qifi('x1'))
    # print(market.get_portfolio())
    settlemsg = market.settle()
    dailyqifi = orjson.loads(settlemsg)
    insertdata.extend(list(dailyqifi.values()))
    market.set_date('2022-07-22')
    market.before_trade()
    market.target_weight({
        "000001": {"000001.XSHE": 0.4, "600010.XSHG": 0.6},
        "000002": {"000001.XSHE": 0.4, "600010.XSHG": 0.6}
    }, 'next_open')
    # print(market.get_account_qifi('x1'))
    # print(market.get_portfolio())
    settlemsg = market.settle()
    dailyqifi = orjson.loads(settlemsg)
    insertdata.extend(list(dailyqifi.values()))

    market.set_date('2022-07-25')
    market.before_trade()
    market.target_weight({
        "000001": {"000001.XSHE": 0.4, "600010.XSHG": 0.6},
        "000002": {"000001.XSHE": 0.4, "600010.XSHG": 0.6}
    }, 'next_open')
    # print(market.get_account_qifi('x1'))
    # print(market.get_portfolio())
    settlemsg = market.settle()
    dailyqifi = orjson.loads(settlemsg)
    insertdata.extend(list(dailyqifi.values()))
    db = pymongo.MongoClient('192.168.2.115').quantaxis.history
    db.create_indexes([pymongo.IndexModel([("account_cookie", pymongo.ASCENDING), ("trading_day", pymongo.ASCENDING), ("portfolio_cookie", pymongo.ASCENDING)], unique=True)])
    db.insert_many(insertdata)
    #print(market.get_mkt_slice())
    #print(market.get_code('000001.XSHE'))

    

if __name__ == "__main__":
    # backtest = qars.Backtest('2022-01-01', '2022-01-21', 'C:\\quant\\data\\mkt.pq')
    # backtest.run(demo_strategy( 'C:\\quant\\data\\mkt.pq'))

    run_market_test()
