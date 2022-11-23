use crate::qaprotocol::qifi::account::QIFI;
use hashbrown::HashMap;

pub struct QA_Portfolio {
    pub portfolio_id: String,
    pub qifis: HashMap<String, QIFI>,
}

impl QA_Portfolio {
    pub fn new(portfolio_id: String, qifis: HashMap<String, QIFI>) -> QA_Portfolio {
        QA_Portfolio {
            portfolio_id,
            qifis,
        }
    }

    pub fn get_portfolio_qifi(&self) -> QIFI {
        let mut qifi: QIFI = self
            .qifis
            .iter()
            .fold(QIFI::default(), |mut qifi, (_, qifi_)| {
                qifi.updatetime = qifi_.updatetime.clone();
                qifi.bankname = qifi_.bankname.clone();
                qifi.bankid = qifi_.bankid.clone();
                qifi.bank_password = qifi_.bank_password.clone();
                qifi.capital_password = qifi_.capital_password.clone();
                qifi.databaseip = qifi_.databaseip.clone();
                qifi.investor_name = qifi_.investor_name.clone();
                qifi.money += qifi_.money;

                qifi.accounts.balance += qifi_.accounts.balance;
                qifi.accounts.available += qifi_.accounts.available;
                qifi.accounts.pre_balance += qifi_.accounts.pre_balance;
                qifi.accounts.deposit += qifi_.accounts.deposit;
                qifi.accounts.withdraw += qifi_.accounts.withdraw;
                qifi.accounts.WithdrawQuota += qifi_.accounts.WithdrawQuota;
                qifi.accounts.close_profit += qifi_.accounts.close_profit;
                qifi.accounts.commission += qifi_.accounts.commission;
                qifi.accounts.premium += qifi_.accounts.premium;
                qifi.accounts.static_balance += qifi_.accounts.static_balance;
                qifi.accounts.position_profit += qifi_.accounts.position_profit;
                qifi.accounts.float_profit += qifi_.accounts.float_profit;
                qifi.accounts.margin += qifi_.accounts.margin;
                qifi.accounts.frozen_margin += qifi_.accounts.frozen_margin;
                qifi.accounts.frozen_commission += qifi_.accounts.frozen_commission;
                qifi.accounts.frozen_premium += qifi_.accounts.frozen_premium;
                qifi.accounts.risk_ratio = qifi.accounts.margin / qifi.accounts.balance;

                let newposkey = qifi_
                    .positions
                    .iter()
                    .map(|(x, _)| x.to_string())
                    .collect::<Vec<String>>();
                let currentkey = qifi
                    .positions
                    .iter()
                    .map(|(x, _)| x.to_string())
                    .collect::<Vec<String>>();

                newposkey.iter().for_each(|key| {
                    if currentkey.contains(key) {
                        qifi.positions.get_mut(key).map(|pos| {
                            let newpos = qifi_.positions.get(key).unwrap();
                            pos.volume_long += newpos.volume_long;
                            pos.volume_long_his += newpos.volume_long_his;
                            pos.volume_long_today += newpos.volume_long_today;
                            pos.volume_long_frozen += newpos.volume_long_frozen;
                            pos.volume_long_frozen_his += newpos.volume_long_frozen_his;
                            pos.volume_long_frozen_today += newpos.volume_long_frozen_today;
                            pos.volume_long_yd += newpos.volume_long_yd;
                            pos.volume_short += newpos.volume_short;
                            pos.volume_short_his += newpos.volume_short_his;
                            pos.volume_short_today += newpos.volume_short_today;
                            pos.volume_short_frozen += newpos.volume_short_frozen;
                            pos.volume_short_frozen_his += newpos.volume_short_frozen_his;
                            pos.volume_short_frozen_today += newpos.volume_short_frozen_today;
                            pos.volume_short_yd += newpos.volume_short_yd;
                            pos.pos_long_his += newpos.pos_long_his;
                            pos.pos_long_today += newpos.pos_long_today;
                            pos.pos_short_his += newpos.pos_short_his;
                            pos.pos_short_today += newpos.pos_short_today;
                            if pos.volume_long + newpos.volume_long > 0.0 {
                                pos.open_price_long = (pos.open_price_long * pos.volume_long
                                    + newpos.open_price_long * newpos.volume_long)
                                    / (pos.volume_long + newpos.volume_long);
                            } else {
                                pos.open_price_long = 0.0;
                            }

                            if pos.volume_short + newpos.volume_short > 0.0 {
                                pos.open_price_short = (pos.open_price_short * pos.volume_short
                                    + newpos.open_price_short * newpos.volume_short)
                                    / (pos.volume_short + newpos.volume_short);
                            } else {
                                pos.open_price_short = 0.0;
                            }

                            pos.open_cost_long += newpos.open_cost_long;
                            pos.open_cost_short += newpos.open_cost_short;
                            if pos.volume_long + newpos.volume_long > 0.0 {
                                pos.position_price_long = (pos.position_price_long
                                    * pos.volume_long
                                    + newpos.position_price_long * newpos.volume_long)
                                    / (pos.volume_long + newpos.volume_long);
                            } else {
                                pos.position_price_long = 0.0;
                            }
                            if pos.volume_short + newpos.volume_short > 0.0 {
                                pos.position_price_short = (pos.position_price_short
                                    * pos.volume_short
                                    + newpos.position_price_short * newpos.volume_short)
                                    / (pos.volume_short + newpos.volume_short);
                            } else {
                                pos.position_price_short = 0.0;
                            }

                            pos.position_cost_long += newpos.position_cost_long;
                            pos.position_cost_short += newpos.position_cost_short;
                            pos.last_price = newpos.last_price;

                            pos.float_profit_long += newpos.float_profit_long;
                            pos.float_profit_short += newpos.float_profit_short;
                            pos.float_profit += newpos.float_profit;
                            pos.position_profit_long += newpos.position_profit_long;
                            pos.position_profit_short += newpos.position_profit_short;
                            pos.position_profit += newpos.position_profit;
                            pos.margin_long += newpos.margin_long;
                            pos.margin_short += newpos.margin_short;
                            pos.margin += newpos.margin;
                        });
                    } else {
                        //println!("{:#?} --- {:#?}", key, qifi_.positions);
                        qifi.positions.insert(key.to_string(), {
                            let mut pos = qifi_.positions.get(key).unwrap().clone();
                            pos.user_id = self.portfolio_id.clone();
                            pos
                        });
                    }
                });

                qifi.orders.extend(qifi_.orders.clone());
                qifi.trading_day = qifi_.trading_day.clone();
                qifi.broker_name = qifi_.broker_name.clone();
                qifi.trades.extend(qifi_.trades.clone());
                qifi
            });
        qifi.account_cookie = self.portfolio_id.clone();
        qifi.portfolio = self.portfolio_id.clone();
        qifi.accounts.user_id = self.portfolio_id.clone();
        qifi
    }
}
