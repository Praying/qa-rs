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
baseprice = 20
mongoclient = pymongo.MongoClient(host='192.168.2.115', port=27017).QAREALTIME.account
simmarket = qars.core.QASIMMarket()
simmarket.init_market(
    {'sim00001': 2000000000, 'sim00002': 500000000, 'sim00003': 2000000000})
simmarket.sync_time()
simacc = ['sim00001', 'sim00002', 'sim00003']
simbs = ['BUY', 'SELL']
#pub= publisher_routing(exchange='CTPX', host='192.168.2.115')
pubx= publisher_topic(exchange='CTPPRO', host='192.168.2.115', durable=True)

for i in range(10000000):
    time.sleep(1)
    currentprice = int(baseprice*(ou_noise()+1)*1000)/1000
    simmarket.insert_order(simacc[random.randint(0, 2)], "IX2301", currentprice, random.randint(
        100, 600), simbs[random.randint(0, 1)], "OPEN")
    simmarket.plot_askbid()
    
    qifi = orjson.loads(simmarket.get_all_account())
    for acc in qifi.values():
        mongoclient.update_one({'account_cookie': acc['account_cookie']}, {'$set': acc}, upsert=True)
    try:
        lasttick = orjson.loads(simmarket.get_lasttick())
        print(lasttick)
        marketpanel= {'ask_price_1': lasttick['ask1'],#bid_volume1
        'ask_price_2':0,
        'ask_price_3':0,
        'ask_price_4':0,
        'ask_price_5':0,
        'ask_volume_1':lasttick['ask_volume1'],
        'ask_volume_2':0,
        'ask_volume_3':0, 
        'ask_volume_4':0, 
        'ask_volume_5':0, 
        'average_price':lasttick['ask_volume1'],
        'bid_price_1': lasttick['bid1'], 
        'bid_price_2':0, 
        'bid_price_3':0,
        'bid_price_4':0,
        'bid_price_5':0,
        'bid_volume_1':lasttick['bid_volume1'],
        'bid_volume_2':0,
        'bid_volume_3':0,
        'bid_volume_4':0,
        'bid_volume_5':0,
        'datetime': str(datetime.datetime.now()), 
        'exchange': 'CFFEX', 
        'gateway_name': 'QASIM', 
        'high_price': 0, 
        'last_price':lasttick['last_price'], 
        'last_volume':lasttick['last_volume'],
        'limit_down': 0,  
        'limit_up': 0, 
        'local_symbol': 'IX2301',
        'low_price':0, 
        'name':'YUT', 
        'open_interest':0, 
        'open_price':0, 
        'preSettlementPrice':20, 
        'pre_close':20, 
        'symbol': 'IX2301',
        'volume':0}
        

            # print(i)
        #pub.pub(json.dumps(marketpanel), routing_key='IX2301')
        pubx.pub(json.dumps(marketpanel), routing_key='IX2301')
    except:
        pass

