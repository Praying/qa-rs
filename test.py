import orjson
import qars

account = qars.QA_QIFIAccount('t', 't', 2000000.0, 'backtest')
account.buy('000001.XSHE', 200.0, '2022-10-21', 2.0)
print(account.get_qifi())

#account.sell('000001.XSHE', 200.0, '2022-10-21', 2.0)
print(account.get_qifi())

print(account.get_floatprofit())
print(account.get_position('000001.XSHE'))


print(account.get_account())

account.settle()
