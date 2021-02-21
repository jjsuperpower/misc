import gym
from gym import spaces
import random
import numpy as np
from math import sin, pi

class Servo_env(gym.Env):
    def __init__(self):
        super(Servo_env, self).__init__()

        self.MIN = -1
        self.MAX = 1

        self.servo1_pos = 0
        self.servo2_pos = 0

        self.action_space = spaces.Box(self.MIN, self.MAX, shape=(1,))
        self.observation_space = spaces.Box(np.array([self.MIN,2*self.MIN]), np.array([self.MAX,2*self.MAX]), shape=(2,))

        self.reset()

    def reset(self):
        self.servo1_pos = random.uniform(self.MIN, self.MAX)
        self.servo2_pos = 0
        new_angle = self.servo1_pos + self.servo2_pos
        return np.array([self.servo2_pos, new_angle])

    def step(self, action):
        angle = self.servo1_pos + action
        reward = 1 - sin(angle * (pi/2))

        self.servo1_pos = random.uniform(self.MIN, self.MAX)
        self.servo2_pos = action
        new_angle = self.servo1_pos + self.servo2_pos
        
        return np.array([self.servo2_pos, new_angle]), reward, False, {}

    def render(self):
        pass

        
        



    
    