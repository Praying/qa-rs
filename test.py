import orjson
import qars
import pymongo
account = qars.QA_QIFIAccount('t', 't', 2000000.0, 'real')
account.buy('000001.XSHE', 200.0, '2022-10-21', 2.0)
history = []
print(account.get_qifi())

print(account.get_floatprofit())
print(account.get_position('000001.XSHE'))
print(account.get_account())
history.append(account.get_qifi())
account.settle()

account.sell('000001.XSHE', 200.0, '2022-10-22', 2.0)
print(account.get_qifi())
history.append(account.get_qifi())

pymongo.MongoClient('192.168.2.115').quantaxis.history.insert_many(history)