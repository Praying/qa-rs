# test  for qars account
import qars
import orjson
import math
class TestAccount:



    def test_acc_target_weight_when_marketvol_limit(self):

        self.market = qars.core.QAMarket('mytestportfolio', '/opt/cache/data/mktdata.pq')
        self.market.init_market({'test_acc': 20000000000000})

        self.market.set_date("2021-03-11")
        self.market.before_trade()
        self.market.target_weight({'test_acc': {'600408.XSHG': 0.8, '600010.XSHG': 0.2}}, 'open')

        pos1 = orjson.loads(self.market.settle())['test_acc']['positions']
        pos1_600408 = pos1['600408.XSHG']
        pos1_600010 = pos1['600010.XSHG']
        #order_book_id	date		limit_down	prev_close	close	open	total_turnover	volume	    low	  limit_up	high	num_trades
        #600408.XSHG	2021-03-11	3.20	     3.37	    3.30	3.41	33971793.0	   10215643.0	3.27	3.54	3.42	1679.0
        
									
        assert pos1_600408['volume_long_today'] == math.floor(10215643*0.0025)*100
        assert pos1_600010['volume_long_today'] == math.floor(2221940988.0*0.0025)*100


    def test_acc_target_weight_when_marketprice_limitdown(self):

        self.market = qars.core.QAMarket('mytestportfolio', '/opt/cache/data/mktdata.pq')
        self.market.init_market({'test_acc': 20000})
        self.market.set_date("2021-03-11")
        self.market.before_trade()
        self.market.target_weight({'test_acc': {'600408.XSHG': 0.8, '600010.XSHG': 0.2}}, 'open')

        pos1 = orjson.loads(self.market.settle())['test_acc']['positions']
        #print(pos1)

        assert pos1['600408.XSHG']['volume_long_today'] == 4600.0
        self.market.set_date("2021-03-12")
        self.market.before_trade()

        self.market.target_weight({'test_acc': {'600408.XSHG':0, '600010.XSHG': 1}}, 'close')
        pos2 = orjson.loads(self.market.settle())['test_acc']['positions']
        #print(pos2)

        assert pos2['600408.XSHG']['volume_long_today'] == 0.0
        ## never sell because 600408 close == limit down
        assert pos2['600408.XSHG']['volume_long'] == 4600.0
        ## frozen has never been release because of 600408 limit down
        assert pos2['600010.XSHG']['volume_long_today'] == 0.0

        self.market.set_date("2021-03-15")
        self.market.before_trade()

        self.market.target_weight({'test_acc': {'600408.XSHG':0, '600010.XSHG': 1}}, 'close')
        pos3 = orjson.loads(self.market.settle())['test_acc']['positions']
        #print(pos2)

        assert pos3['600408.XSHG']['volume_long_today'] == -4600.0
        ## never sell
        assert pos3['600408.XSHG']['volume_long'] == 0.0
        assert pos3['600010.XSHG']['volume_long_today'] == 5600.0
        