import qars

account = qars.core.QA_Account('t','t',2000000.0, 'backtest')
account.buy('000001.XSHE', 200.0, '2022-10-21', 2.0)
print(account.get_qifi())