import qars


class Backtest():
    def __init__(self, start, end, path) -> None:
        self.backtest = qars.core.QABacktest(start, end, path)

    def run(self, strategy: qars.core.StrategyWrapper):
        self.backtest.run(qars.core.StrategyWrapper(strategy))
