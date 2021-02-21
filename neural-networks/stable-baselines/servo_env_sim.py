import gym
from gym import spaces
import random
import numpy as np
from math import sin, pi

class Servo_Env_Sim(gym.Env):
    def __init__(self):
        super(Servo_Env_Sim, self).__init__()

        self.MIN = -1.0
        self.MAX = 1.0

        self.servo_ang = 0.0     #rotation of servo for simulation
        self.servo_pos = 0.0     #position of servo horn
        self.sensor_ang = 0.0     #angle of sensor

        #one action demintion = servo pos
        self.action_space = spaces.Box(self.MIN, self.MAX, shape=(1,), dtype=np.float)      
        #two obs demintion = servo current pos, sensor angle
        self.observation_space = spaces.Box(np.array([self.MIN,self.MIN]),np.array([self.MAX,self.MAX]), shape=(2,))

        self.reset()

    def reset(self):
        self.servo_ang = random.uniform(self.MIN, self.MAX)
        self.servo_pos = random.uniform(self.MIN, self.MAX)
        self.sensor_ang = self.servo_ang + self.servo_pos
        return np.array([self.sensor_ang, self.servo_pos])

    def step(self, action):
        angle = self.servo_ang + action
        reward = 1 - abs(sin(angle * (pi/2)))

        return self.reset(), reward, False, {}

    def render():
        pass
