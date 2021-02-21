import socket
import pickle
import numpy as np
import gym
from gym import spaces
from time import sleep, time

import os


class Servo_Env_Real(gym.Env):
    def __init__(self):
        super().__init__
        self.connect = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.SOCKET_BUF_SIZE = 32_000
        self.connect.setblocking(1)
        self.connect.connect(('10.42.0.164', 5555))
        print(self.connect.recv(self.SOCKET_BUF_SIZE).decode())

        self.MIN = -1.0
        self.MAX = 1.0

        self.servo_ang = 0.0     #rotation of servo for simulation
        self.servo_pos = 0.0     #position of servo horn
        self.sensor_ang = 0.0     #angle of sensor

        #one action demintion = servo pos
        self.action_space = spaces.Box(self.MIN, self.MAX, shape=(1,), dtype=np.float)      
        #two obs demintion = servo current pos, sensor angle
        self.observation_space = spaces.Box(np.array([self.MIN,self.MIN]),np.array([self.MAX,self.MAX]), shape=(2,))

    def close(self):
        self.connect.send(bytes("stop", "utf-8"))
        sleep(.05)
        self.connect.recv(self.SOCKET_BUF_SIZE)

    def reset(self):
        self.connect.send(bytes("reset", "utf-8"))
        sleep(.05)
        data = pickle.loads(self.connect.recv(self.SOCKET_BUF_SIZE))
        return data

    def step(self, actions):
        # print('Action')
        self.connect.send(bytes("step", "utf-8"))
        sleep(.05)
        self.connect.send(pickle.dumps(actions))
        raw_data = self.connect.recv(self.SOCKET_BUF_SIZE)
        data = pickle.loads(raw_data)
        obs, reward, done = data
        return obs, reward, done, {}