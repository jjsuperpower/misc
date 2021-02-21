import chainer
from chainer import functions as F
from chainer import links as L
import numpy as np

import chainerrl
from chainerrl.agents import PPO
from chainerrl import experiments
from chainerrl import misc
env = Servo_env()
obs = env.reset()

class QFunction(chainer.Chain):
    def __init__(self, obs_size, n_actions, n_hidden_channels=50):
        super().__init__()
        with self.init_scope():
            self.l0 = L.Linear(obs_size, n_hidden_channels)
            self.l1 = L.Linear(n_hidden_channels, n_hidden_channels)
            self.l2 = L.Linear(n_hidden_channels, n_actions)
    # def __call__(self, x, test=False):
    #     h = F.tanh(self.l0(x))
    #     h = F.tanh(self.l1(h))
    #     return chainerrl.action_value.

obs_size = env.observation_space.shape[0]
n_actions = env.action_space.n
q_func = QFunction(obs_size, n_actions=1)