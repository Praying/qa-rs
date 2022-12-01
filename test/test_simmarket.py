#!/usr/bin/env python
# coding: utf-8

# In[31]:

import json
from QAPUBSUB.producer import publisher_routing, publisher_topic
import threading

import random
import qars
import pymongo
import polars as pl
import orjson
import numpy as np
import QAPUBSUB.producer as producer
import time
import datetime


class OrnsteinUhlenbeckActionNoise:

    def __init__(self, mu=0, sigma=0.2, theta=0.15, dt=1e-2, x0=None):
        self.theta = theta
        self.mu = mu
        self.sigma = sigma
        self.dt = dt
        self.x0 = x0
        self.reset()

    def __call__(self):
        x = self.x_prev + self.theta * (self.mu - self.x_prev) * self.dt + \
            self.sigma * np.sqrt(self.dt) * \
            np.random.normal(size=self.mu.shape)

        self.x_prev = x
        return x

    def reset(self):
        self.x_prev = self.x0 if self.x0 is not None else np.zeros_like(
            self.mu)

    def __repr__(self):
        return 'OrnsteinUhlenbeckActionNoise(mu={}, sigma={})'.format(self.mu, self.sigma)


ou_noise = OrnsteinUhlenbeckActionNoise(mu=np.array(0))


simbs = ['BUY', 'SELL']
#pub= publisher_routing(exchange='CTPX', host='192.168.2.27')

class QASIM():
    def __init__(self, account_num) -> None:
        self.accounts = dict(zip(['9999{}'.format(i)
                for i in range(10, account_num+10)], [1000000] * account_num))
        self.simmarket = qars.core.QASIMMarket()
        self.simmarket.init_market(self.accounts)
        self.simmarket.sync_time()

        self.simacc = list(self.accounts.keys())
        self.pubx = publisher_topic(exchange='CTPPRO', host='192.168.2.115', durable=True)
        self.available_obi = ['IX2301', 'IY2301', 'IZ2301']
        self.mongoclient = pymongo.MongoClient(
                host='192.168.2.27', port=27017).QAREALTIME.account


        self.init_market= dict(zip(self.available_obi, 
            [{'low':20, 'high':20, 'total_volume':0, 'last_volume':0, 'last_price':20, 'preclose': 20}, 
            {'low':30, 'high':30, 'total_volume':0, 'last_volume':0, 'last_price':30, 'preclose': 30},
            {'low':60, 'high':60, 'total_volume':0, 'last_volume':0, 'last_price':60, 'preclose': 60}]))

    def save(self):
        qifi = orjson.loads(self.simmarket.get_all_account())
        for acc in qifi.values():
            self.mongoclient.update_one({'account_cookie': acc['account_cookie']}, {
                                '$set': acc}, upsert=True)

    def run(self):
        for i in range(10000000):
            time.sleep(1)
            for ob in self.available_obi:
                self.single_market(ob)
            self.save()
            self.simmarket.plot_askbid()

    def single_market(self, code='IX2301'):
        lastprice = self.init_market[code]['last_price']
        currentprice = int(lastprice*(int(ou_noise()*100)/100000+1)*100)/100
        print(currentprice)
        self.simmarket.insert_order(self.simacc[random.randint(0, len(self.simacc)-1)], code, currentprice, random.randint(
            100, 600), simbs[random.randint(0, 1)], "OPEN")
        


        try:
            lasttick = orjson.loads(self.simmarket.get_lasttick())[code]
            self.init_market[code]['last_volume'] += lasttick['last_volume']
            if lasttick['last_price']>0:
                self.init_market[code]['total_volume'] += lasttick['last_volume']*lasttick['last_price']
                if self.init_market[code]['low'] > lasttick['last_price']:
                    self.init_market[code]['low'] = lasttick['last_price']
                if self.init_market[code]['high'] < lasttick['last_price']:
                    self.init_market[code]['high'] = lasttick['last_price']
                marketpanel = {'ask_price_1': lasttick['ask1'],  # bid_volume1
                            'ask_price_2': 0,
                            'ask_price_3': 0,
                            'ask_price_4': 0,
                            'ask_price_5': 0,
                            'ask_volume_1': lasttick['ask_volume1'],
                            'ask_volume_2': 0,
                            'ask_volume_3': 0,
                            'ask_volume_4': 0,
                            'ask_volume_5': 0,
                            'average_price': self.init_market[code]['total_volume']/self.init_market[code]['last_volume'],
                            'bid_price_1': lasttick['bid1'],
                            'bid_price_2': 0,
                            'bid_price_3': 0,
                            'bid_price_4': 0,
                            'bid_price_5': 0,
                            'bid_volume_1': lasttick['bid_volume1'],
                            'bid_volume_2': 0,
                            'bid_volume_3': 0,
                            'bid_volume_4': 0,
                            'bid_volume_5': 0,
                            'datetime': str(datetime.datetime.now())[0:19],
                            'exchange': 'CFFEX',
                            'gateway_name': 'QASIM',
                            'high_price': self.init_market[code]['high'],
                            'last_price': lasttick['last_price'],
                            'last_volume': lasttick['last_volume'],
                            'limit_down': 0,
                            'limit_up': 0,
                            'local_symbol': code,
                            'low_price': self.init_market[code]['low'],
                            'name': 'IX',
                            'open_interest': self.init_market[code]['total_volume'],
                            'open_price': self.init_market[code]['preclose'],
                            'preSettlementPrice': self.init_market[code]['preclose'],
                            'pre_close': self.init_market[code]['preclose'],
                            'symbol': code,
                            'volume': self.init_market[code]['last_volume']}

                #print('----------', marketpanel)
                # print(i)
                #pub.pub(json.dumps(marketpanel), routing_key='IX2301')
                self.init_market[code]['last_price'] = currentprice

                self.pubx.pub(json.dumps(marketpanel), routing_key=code)
        except Exception as e:
            print(e)
                



if __name__=="__main__":
    qasim = QASIM(50)
    qasim.run()