import orjson
from . import qars_core as core
import sys
sys.path.append('./')
sys.path.append('../')


class QA_QIFIAccount():

    def __init__(self, account_name, portfolio_name, init_cash, environment):
        self.account = core.QA_Account(
            account_name, portfolio_name, init_cash, environment)

    def buy(self, code, price, date, amount):
        return self.account.buy(code, price, date, amount)

    def sell(self, code, price, date, amount):
        return self.account.sell(code, price, date, amount)

    def buy_open(self, code, price, date, amount):
        return self.account.buy_open(code, price, date, amount)

    def buy_close(self, code, price, date, amount):
        return self.account.buy_close(code, price, date, amount)

    def buy_closetoday(self, code, price, date, amount):
        return self.account.buy_closetoday(code, price, date, amount)

    def sell_open(self, code, price, date, amount):
        return self.account.sell_open(code, price, date, amount)

    def sell_close(self, code, price, date, amount):
        return self.account.sell_close(code, price, date, amount)

    def sell_closetoday(self, code, price, date, amount):
        return self.account.sell_closetoday(code, price, date, amount)

    def get_qifi(self):
        return orjson.loads(self.account.get_qifi())

    def get_floatprofit(self):
        return self.account.get_floatprofit()

    def get_position(self, code):
        return orjson.loads(self.account.get_position(code))

    def get_account(self):
        return orjson.loads(self.account.get_account())

    def settle(self):
        self.account.settle()

    def get_positions(self):
        return orjson.loads(self.account.get_positions())

    def dividend_event(self, ratio):
        self.account.dividend_event(ratio)
    
    def split_event(self, code, ratio):
        self.account.split_event(code, ratio)



# class Strategy(core.Context):

#     # 策略运行前初始化
#     def on_init(self):
#         self.last_id = 0
#         self.num = 6

#         # 开仓间隔
#         self.next_open_days = 1
#         # 持仓天数
#         self.hold_period = 1
#         # 单日最大资金占用率
#         self.max_margin_use = 1


#         #self.percent = self.get_percent(self.hold_period, self.next_open_days, self.max_margin_use, 'close', 'close', False)

#     # 核心逻辑,同一时间戳全部bar结束方可触发

#     def on_calc(self):

#         self.clear_timeline('close')

#         if self.last_id == 0 or self.last_id == self.bar_id:
#             # print(self.datetime)
#             table = self.get_map('code', 'weight_div_open', 0)
            
#             lastbalance = self.pre_balance-12000000000
#             # print(self.date,lastbalance)
#             targetp = {}
#             for x in table.keys():
#                 if table[x] > 0:

#                     targetp[x] = int(lastbalance*table[x]/100-1)*100
#             # print(targetp)
#                 if table[x] <0:
#                     targetp[x] = -1*int(lastbalance*abs(table[x])/100-1)*100

#             self.target_byorder(targetp, 'open')

#             self.last_id = self.bar_id + self.next_open_days
