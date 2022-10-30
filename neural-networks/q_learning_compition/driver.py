import gym
import random
import numpy as np

env = gym.make('CartPole-v1')
env.reset()


ACTION_SPACE = [0, 1]

action_replay_mem = []
epsilon = .5
epsilon_decay = .01     #decay per step


def get_action():
    global epsilon
    epsilon -= epsilon_decay

    if random.random() < epsilon:
        return random.choice(ACTION_SPACE)

    else:
        return 1

def store_action(state, action, reward):
    global action_replay_mem
    action_replay_mem.append((state, action, reward))
    action_replay_mem = action_replay_mem[:1000]


state = np.empty(4, dtype=np.float32)
reward = 0

for _ in range(20):        # number of episodes
    while(True):

        env.render()

        action = get_action()
        old_state = state
        state, reward, done, _ = env.step(action)
        store_action(old_state, action, reward)

        if done:
            env.reset()
            break

env.close()
