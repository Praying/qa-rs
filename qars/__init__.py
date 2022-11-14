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
