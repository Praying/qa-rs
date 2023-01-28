# test  for qars account
import qars
import orjson
import math
import pandas as pd
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
        print('test_acc_target_weight_when_marketprice_limitdown in 2021/3/11')
        #print(pos1)
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))
        assert pos1['600408.XSHG']['volume_long_today'] == 4600.0
        self.market.set_date("2021-03-12")
        self.market.before_trade()

        self.market.target_weight({'test_acc': {'600408.XSHG':0, '600010.XSHG': 1}}, 'close')
        pos2 = orjson.loads(self.market.settle())['test_acc']#['positions']
        #print(pos2)
        print('test_acc_target_weight_when_marketprice_limitdown in 2021/3/12')
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))
        
        print(pos2['trades'])
        assert pos2['positions']['600408.XSHG']['volume_long_today'] == 0.0
        ## never sell because 600408 close == limit down
        assert pos2['positions']['600408.XSHG']['volume_long'] == 4600.0
        ## frozen has never been release because of 600408 limit down
        assert pos2['positions']['600010.XSHG']['volume_long_today'] == 200.0

        self.market.set_date("2021-03-15")
        self.market.before_trade()

        self.market.target_weight({'test_acc': {'600408.XSHG':0, '600010.XSHG': 1}}, 'close')
        pos3 = orjson.loads(self.market.settle())['test_acc']['positions']
        #print(pos2)

        assert pos3['600408.XSHG']['volume_long_today'] == -4600.0
        ## never sell
        assert pos3['600408.XSHG']['volume_long'] == 0.0
        assert pos3['600010.XSHG']['volume_long_today'] == 7400.0
        print('test_acc_target_weight_when_marketprice_limitdown in 2021/3/15')
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))

    def test_acc_target_weight_when_one_code_limit_by_vol_and_notfree_cash(self):
        self.market = qars.core.QAMarket('mytestportfolio', '/opt/cache/data/mktdata.pq')
        self.market.init_market({'test_acc': 20000000})
        self.market.set_date("2021-03-11")
        self.market.before_trade()
        self.market.target_weight({'test_acc': {'600408.XSHG': 0.7,'000001.XSHE': 0.1, '600010.XSHG': 0.1}}, 'open')
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))
        trade = pd.DataFrame(orjson.loads(self.market.settle())['test_acc']['trades'])
        print(trade)

        print('\n\n\\2021-03-12\n\n')
        self.market.set_date("2021-03-12")
        self.market.before_trade()
        self.market.target_weight({'test_acc': {'600408.XSHG': 0.7,'000001.XSHE': 0.1, '600010.XSHG': 0.1}}, 'open')
        
        trade = pd.DataFrame(orjson.loads(self.market.settle())['test_acc']['trades'])
        print(trade)
        hold = pd.DataFrame(orjson.loads(self.market.settle())['test_acc']['positions'].values())
        print(hold)
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))
        # 600010 volume 10752600.0
        # 000001 volume 935800.0
        # 600408 volume 4198800.0
 
        print('\n\n\\2021-03-15\n\n')
        self.market.set_date("2021-03-15")
        self.market.before_trade()
        self.market.target_weight({'test_acc': {'600408.XSHG': 0.1,'000001.XSHE': 0.45, '600010.XSHG': 0.45}}, 'open')
        

        trade = pd.DataFrame(orjson.loads(self.market.settle())['test_acc']['trades'])
        print(trade)
        # 600010 volume 15557700.0
        # 000001 volume  1240300.0
        # 600408 volume 330500.0

        hold = pd.DataFrame(orjson.loads(self.market.settle())['test_acc']['positions'].values())
        print(hold)
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))

        print('\n\n\\2021-03-16\n\n')
        self.market.set_date("2021-03-16")
        self.market.before_trade()
        self.market.target_weight({'test_acc': {'600408.XSHG': 0,'000001.XSHE': 0.5, '600010.XSHG': 0.5}}, 'open')
        
        trade = pd.DataFrame(orjson.loads(self.market.settle())['test_acc']['trades'])
        print(trade)
        print(self.market.get_pos_value('test_acc'))
        print(self.market.get_pos_weight('test_acc'))
