# Account

原生支持跨市场的qaaccount

## 创建一个account
```python
account = qars.QA_QIFIAccount(account_name, portfolio_name, init_cash,  model)
```
参数

- account_name: 账户名 string
- portfolio_name: 组合名 string
- init_cash: 账户初始资金 float
- model: 模型(回测backtest/ 实盘 real)  

回测和实盘的区别主要是存在于对于history order/trade的处理, 如果你在回测模式中也想要处理完整的qifi截面, 获得order/trade, 可以主要使用real模式

## 下单函数
```python
account.buy('000001.XSHE', 200.0, '2022-10-21', 2.0)
account.buy_open('000001.XSHE', 200.0, '2022-10-21', 2.0)
account.sell_open('000001.XSHE', 200.0, '2022-10-21', 2.0)
account.sell('000001.XSHE', 200.0, '2022-10-21', 2.0)
account.sell_close('000001.XSHE', 200.0, '2022-10-21', 2.0)
account.buy_close('000001.XSHE', 200.0, '2022-10-21', 2.0)

account.target({'000001.XSHE', 200})
```

```
import orjson
import qars
import pymongo
account = qars.QA_QIFIAccount('t2', 't', 2000000.0, 'real')

history = []
print(account.get_qifi())

print(account.get_floatprofit())
print(account.get_position('000001.XSHE'))
print(account.get_account())
history.append(account.get_qifi())
account.settle()


account.split_event('000001.XSHE', 0.2)
account.sell('000001.XSHE', 200.0, '2022-10-24', 2.0)
history.append(account.get_qifi())
account.settle()

account.sell_open('IC99', 2.0, '2022-10-25', 3000.0)
print(account.get_qifi())
history.append(account.get_qifi())
```