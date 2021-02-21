import chainer
import chainer.functions as F
import chainer.links as L

import chainerrl
from chainerrl.agents import PPO
from chainerrl import experiments
from chainerrl import misc

import gym
import numpy as np


env = gym.make('MountainCarContinuous-v0')
print('observation space:', env.observation_space)
print('action space:', env.action_space)

obs = env.reset()
env.render()
print('initial observation:', obs)

action = env.action_space.sample()
obs, r, done, info = env.step(action)
print('next observation:', obs)
print('reward:', r)
print('done:', done)
print('info:', info)

obs_size = env.observation_space.shape[0]
n_hidden_channels = 20
#n_actions = env.action_space.shape[0]
n_actions = 1

model = chainer.Sequential(
    L.Linear(obs_size, n_hidden_channels),
    F.relu,
    L.Linear(n_hidden_channels, n_hidden_channels),
    F.relu,
    chainerrl.links.Branched(
        chainer.Sequential(
            L.Linear(None, n_actions),
            chainerrl.distribution.SoftmaxDistribution,
        ),
        L.Linear(None, 1),
    )
)


opt = chainer.optimizers.Adam()
opt.setup(model)
opt.add_hook(chainer.optimizer.GradientClipping(0.5))

def phi(x):
        # Feature extractor
        return np.asarray(x, dtype=np.float32)

agent=PPO(
    model,
    opt,
    phi=phi
)

# experiments.train_agent_with_evaluation(
#     agent=agent,
#     steps=2000,
#     env=env,
#     eval_n_steps=None,
#     eval_max_episode_len=100,
#     eval_n_episodes=5,
#     eval_interval=3,
#     outdir="test2"
# )




# Set the discount factor that discounts future rewards.
gamma = 0.95

# Use epsilon-greedy for exploration
explorer = chainerrl.explorers.ConstantEpsilonGreedy(
    epsilon=0.3, random_action_func=env.action_space.sample)

n_episodes = 200
max_episode_len = 200
for i in range(1, n_episodes + 1):
    obs = env.reset()
    reward = 0
    done = False
    R = 0  # return (sum of rewards)
    t = 0  # time step
    while not done and t < max_episode_len:
        # Uncomment to watch the behaviour
        # env.render()
        action = agent.act_and_train(obs, reward)
        obs, reward, done, _ = env.step(action)
        R += reward
        t += 1
    if i % 10 == 0:
        print('episode:', i,
              'R:', R,
              'statistics:', agent.get_statistics())
    agent.stop_episode_and_train(obs, reward, done)
print('Finished.')


