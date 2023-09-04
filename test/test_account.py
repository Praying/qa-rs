import qars
import orjson
import pytest
import pytest_cov
import pytest_html
class TestAcc:

    def test_acc_settle(self):
        print('test_acc_settle')
        #        account_cookie: &str,
        # portfolio_cookie: &str,
        # init_cash: f64,
        # environment: &str,
        acc = qars.core.QA_Account('test_acc', 'portfolio', 20000000, "real")
        acc.buy_open("000001.XSHE", 200,'2021-01-01', 20)
        acc.on_pricechange("000001.XSHE", 21, '2021-01-01')

        res = orjson.loads(acc.get_qifi())
        assert res['accounts']['balance'] == 20000198.72
        assert res['accounts']['pre_balance'] == 20000000.0
        assert res['accounts']['static_balance'] == 20000000.0
        assert res['accounts']['margin'] == 4000.0
        assert res['accounts']['available'] == 19995998.72
        assert res['accounts']['static_balance'] + res['accounts']['float_profit'] + res['accounts']['close_profit'] - res['accounts']['commission'] == res['accounts']['pre_balance'] + res['accounts']['position_profit'] - res['accounts']['commission']
        print(res['accounts'])

        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20000000.0, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 20000000.0, 'close_profit': 0.0, 'commission': 1.28, 'premium': 0.0, 'static_balance': 20000000.0, 'position_profit': 200.0, 'float_profit': 200.0, 'balance': 20000198.72, 'margin': 4000.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19995998.72, 'risk_ratio': 0.0002}

        acc.settle()
        resx = orjson.loads(acc.get_qifi())
        assert resx['accounts']['balance'] == 20000198.72
        assert resx['accounts']['pre_balance'] == 20000198.72
        assert resx['accounts']['static_balance'] == 19999998.72
        print(resx['accounts'])
        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20000198.72, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19995998.72, 'close_profit': 0.0, 'commission': 0.0, 'premium': 0.0, 'static_balance': 19999998.72, 'position_profit': 0.0, 'float_profit': 200.0, 'balance': 20000198.72, 'margin': 4000.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19995998.72, 'risk_ratio': 0.00019999801281974465}
        #print(res)



        ###day2
        #acc.buy_open("000001", 200,'2021-01-01', 20,)
        acc.on_pricechange("000001.XSHE", 22, '2021-01-02')
        res = orjson.loads(acc.get_qifi())
        assert res['accounts']['balance'] == 20000398.72
        assert res['accounts']['pre_balance'] == 20000198.72
        assert res['accounts']['static_balance'] == 19999998.72
        assert res['accounts']['static_balance'] + res['accounts']['float_profit'] + res['accounts']['close_profit'] - res['accounts']['commission'] == res['accounts']['pre_balance'] + res['accounts']['position_profit'] - res['accounts']['commission']
        assert res['positions']['000001.XSHE']['position_price_long'] == 21.0
        assert res['positions']['000001.XSHE']['open_price_long'] == 20.0
        assert res['positions']['000001.XSHE']['position_cost_long'] == 4200.0
        assert res['positions']['000001.XSHE']['float_profit'] == 400.0
        assert res['positions']['000001.XSHE']['position_profit'] == 200.0
        assert res['positions']['000001.XSHE']['margin'] == 4000.0
        print(res['accounts'])
        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20000198.72, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19995998.72, 'close_profit': 0.0, 'commission': 0.0, 'premium': 0.0, 'static_balance': 19999998.72, 'position_profit': 200.0, 'float_profit': 400.0, 'balance': 20000398.72, 'margin': 4000.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19995998.72, 'risk_ratio': 0.00019999801281974465}
        acc.settle()
        resx = orjson.loads(acc.get_qifi())
        assert resx['accounts']['balance'] == 20000398.72
        assert resx['accounts']['pre_balance'] == 20000398.72
        assert resx['accounts']['static_balance'] == 19999998.72
        print(resx['accounts'])
        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20000398.72, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19995998.72, 'close_profit': 0.0, 'commission': 0.0, 'premium': 0.0, 'static_balance': 19999998.72, 'position_profit': 0.0, 'float_profit': 400.0, 'balance': 20000398.72, 'margin': 4000.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19995998.72, 'risk_ratio': 0.00019999601287948725}



        ####day3
        acc.buy_open("000001.XSHE", 200, '2021-01-03',23)
        acc.on_pricechange("000001.XSHE", 24, '2021-01-03')
        res = orjson.loads(acc.get_qifi())
        print(res['accounts'])
        assert res['accounts']['balance'] == 20000997.248
        assert res['accounts']['pre_balance'] == 20000398.72
        assert res['accounts']['static_balance'] == 19999998.72
        assert res['accounts']['static_balance'] + res['accounts']['float_profit'] + res['accounts']['close_profit'] - res['accounts']['commission'] == res['accounts']['pre_balance'] + res['accounts']['position_profit'] - res['accounts']['commission']
        
        assert res['positions']['000001.XSHE']['position_price_long'] == 22.5
        assert res['positions']['000001.XSHE']['open_price_long'] == 21.5
        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20000398.72, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19995998.72, 'close_profit': 0.0, 'commission': 1.4720000000000002, 'premium': 0.0, 'static_balance': 19999998.72, 'position_profit': 600.0, 'float_profit': 1000.0, 'balance': 20000997.248, 'margin': 8600.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19991397.248, 'risk_ratio': 0.0004299914276908976}
        acc.settle()
        resx = orjson.loads(acc.get_qifi())
        assert resx['accounts']['balance'] == 20000997.248
        assert resx['accounts']['pre_balance'] == 20000997.248
        assert resx['accounts']['static_balance'] ==  19999997.248
        print(resx['accounts'])
        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20000997.248, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19991397.248, 'close_profit': 0.0, 'commission': 0.0, 'premium': 0.0, 'static_balance': 19999997.248, 'position_profit': 0.0, 'float_profit': 1000.0, 'balance': 20000997.248, 'margin': 8600.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19991397.248, 'risk_ratio': 0.00042997856023703804}

        #print(res['positions']['000001.XSHE'])


        ####day4

        acc.sell_close("000001.XSHE", 200, '2021-01-04', 25)
        acc.on_pricechange("000001.XSHE", 26, '2021-01-04')
        res = orjson.loads(acc.get_qifi())
        print(res['accounts'])

        assert res['accounts']['balance'] == 20001590.648
        assert res['accounts']['pre_balance'] == 20000997.248
        assert res['accounts']['static_balance'] == 19999997.248
        assert res['accounts']['close_profit'] == 700.0
        assert res['accounts']['position_profit'] == 600.0
        assert res['accounts']['static_balance'] + res['accounts']['float_profit'] + res['accounts']['close_profit'] - res['accounts']['commission'] == res['accounts']['pre_balance'] + res['accounts']['position_profit'] - res['accounts']['commission']
        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20001590.648, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19996390.648, 'close_profit': 0.0, 'commission': 0.0, 'premium': 0.0, 'static_balance': 20000690.648, 'position_profit': 200.0, 'float_profit': 900.0, 'balance': 20001590.648, 'margin': 4300.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19996390.648, 'risk_ratio': 0.00021498290189385345}
        acc.settle()
        resx = orjson.loads(acc.get_qifi())
        print(resx['accounts'])


        {'user_id': 'test_acc', 'currency': 'CNY', 'pre_balance': 20001590.648, 'deposit': 0.0, 'withdraw': 0.0, 'WithdrawQuota': 19996390.648, 'close_profit': 0.0, 'commission': 0.0, 'premium': 0.0, 'static_balance': 20000690.648, 'position_profit': 0.0, 'float_profit': 900.0, 'balance': 20001590.648, 'margin': 4300.0, 'frozen_margin': 0.0, 'frozen_commission': 0.0, 'frozen_premium': 0.0, 'available': 19996390.648, 'risk_ratio': 0.00021498290189385345}
        assert resx['accounts']['balance'] == 20001590.648
        assert resx['accounts']['pre_balance'] == 20001590.648
        assert resx['accounts']['static_balance'] == 20000690.648
        assert resx['accounts']['close_profit'] == 0.0
        assert resx['accounts']['static_balance'] + resx['accounts']['float_profit'] + resx['accounts']['close_profit'] - resx['accounts']['commission'] == resx['accounts']['pre_balance'] + resx['accounts']['position_profit'] - resx['accounts']['commission']