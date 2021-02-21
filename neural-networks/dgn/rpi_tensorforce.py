# import keras.backend.tensorflow_backend as backend
# from keras.models import Sequential
# from keras.layers import Dense, Dropout, Conv2D, MaxPooling2D, Activation, Flatten
# from keras.optimizers import Adam
# from keras.callbacks import TensorBoard

# import tensorflow as tf

from tensorforce.agents import Agent
from tensorforce.environments import Environment
from tensorforce.execution import Runner

import socket
import pickle
# import math
from time import sleep, time
# import random
# import numpy as np
import os

print('Imported Libraries')

# stream = os.popen('tensorboard --logdir data/summaries/')
# print('Starting tensorboard')

class Enviroment_API(Environment):
    def __init__(self):
        super().__init__

        self.connect = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.connect.connect(('raspberrypi.sanderson.com', 5555))
        print(self.connect.recv(1024).decode())


    def states(self):
        return dict(type='float', shape=(3,))

    def actions(self):
        return dict(type='float', shape=(1,), min_value=0.0, max_value=1.0)

    def max_episode_timesteps(self):
        return None

    def close(self):
        self.connect.send(bytes("stop", "utf-8"))
        self.connect.recv(1024)

    def reset(self):
        self.connect.send(bytes("reset", "utf-8"))
        data = pickle.loads(self.connect.recv(1024))
        return dict(state=data)

    def execute(self, actions):
        # print('Action')
        self.connect.send(bytes("step", "utf-8"))
        self.connect.send(pickle.dumps(actions))
        data = pickle.loads(self.connect.recv(1024))
        new_state, reward, done = data
        return dict(state=new_state), done, reward



AGENT = 'ppo'

env = Environment.create(environment='__main__.Enviroment_API')
agent = Agent.create(
                    agent=AGENT,
                    environment=env,
                    max_episode_timesteps=20,
                    exploration=.3,
                    batch_size=5,
                    # memory=100,
                    summarizer=dict(directory='data/summaries', frequency=1),
                    saver=dict(directory='data/agent', frequency=60)
                    )

runner = Runner(
                agent=agent,
                environment=env
)

runner.run(num_episodes=100)
runner.run(num_episodes=20, evaluation=True)
runner.close()
