from mpu6050 import mpu6050
import RPi.GPIO as GPIO

import keras.backend.tensorflow_backend as backend
from keras.models import Sequential
from keras.layers import Dense, Dropout, Conv2D, MaxPooling2D, Activation, Flatten
from keras.optimizers import Adam
from keras.callbacks import TensorBoard

import math
from time import sleep, time
import random
import numpy as np
from selections import deque

LEARNING_RATE = .2
DISCOUNT = .8
EPISODES = 1000
EPSILON_START = .05
EPSILON_END = .01

EPISLON_DECAY_RATE = (EPSILON_START - EPSILON_END) / EPISODES

class Enviroment:
    def __init__(self):

        self.ACCEL_SAMPLE_SIZE = 32
        self.GYRO_SAMPLE_SIZE = 32
        self.SENSOR_TOLERANCE = 1
        self.SENSOR_TARGET_POS = 0

        self.SERVO_0_PIN = 22
        self.SERVO_1_PIN = FIX
        self.SERVO_FREQ = 50
        self.SERVO_DUTY_MIN = 3
        self.SERVO_DUTY_MAX = 11
        self.SERVO_TIME_DELAY_MIN = .2
        self.SERVO_TIME_DELAY_MAX = .7

        self.ACTION_REWARD_FACTOR = 100
        self.ACTION_REWARD_MAX = 150
        self.TIME_PEN_FACTOR = 0

        GPIO.setwarnings(False)
        GPIO.setmode(GPIO.BOARD)
        GPIO.setup(self.SERVO_0_PIN, GPIO.OUT)
        GPIO.setup(self.SERVO_1_PIN, GPIO.OUT)

        self.servo_0 = GPIO.PWM(self.SERVO_0_PIN, self.SERVO_FREQ)
        self.servo_1 = GPIO.PWM(self.SERVO_1_PIN, self.SERVO_FREQ)
        self.sensor = mpu6050(0x68)

        self.servo_0_pos = 0
        self.servo_1_pos = 0
        self.time_start = 0
        self.time_stop = 0

    def servo_set_pos(self, servo, servo_current_pos, servo_new_pos):
        if(0 <= servo_new_pos <= 1):
            servo.ChangeDutyCycle((self.SERVO_DUTY_MAX - self.SERVO_DUTY_MIN) * servo_new_pos + self.SERVO_DUTY_MIN)
            
        delay = abs(servo_new_pos - servo_current_pos) * self.SERVO_TIME_DELAY_MAX
        if (delay < self.SERVO_TIME_DELAY_MIN):
            delay = self.SERVO_TIME_DELAY_MIN
        if (delay > self.SERVO_TIME_DELAY_MAX):
            delay = self.SERVO_TIME_DELAY_MAX
        sleep(delay)

    def reset(self):
        servo_new_pos = np.random.random()
        self.servo_set_pos(self.servo_0_pos, self.servo_0_pos, servo_new_pos)
        self.servo_0_pos = servo_new_pos
        self.time_start = time()

    def stop(self):
        self.servo_0_pos.stop()
        self.servo_0_pos.stop()

    def get_accel_avg(self):
        accel_list = []
        for i in range(self.ACCEL_SAMPLE_SIZE):
            accel_list.append(self.sensor.get_accel_data()['y'])
        return sum(accel_list) / len(accel_list)
        
    def get_gyro_avg(self):
        gyro_list = []
        for i in range(self.GYRO_SAMPLE_SIZE):
            gyro_list.append(self.sensor.get_gyro_data()['y'])
        return sum(gyro_list) / len(gyro_list)

    def step(self, action):

        self.servo_set_pos(self.servo_1, self.servo_1_pos, action)
        self.servo_1_pos = action

        reward = (self.SENSOR_TOLERANCE / abs(self.SENSOR_TARGET_POS - self.get_accel_avg())) * self.ACTION_REWARD_FACTOR

        if (reward > self.ACTION_REWARD_MAX):
            reward = self.ACTION_REWARD_MAX

        if (reward >= self.ACTION_REWARD_FACTOR):

            self.time_stop = time()
            time_dif = (self.time_stop - self.time_start) * self.TIME_PEN_FACTOR

            reward += self.ACTION_REWARD_FACTOR
            penalty = time_dif * self.TIME_PEN_FACTOR
            done = True

        else:
            done = False

        print('Reward = ' + str(int(reward)))

        new_state = [self.get_accel_avg(), self.servo_1_pos, self.SENSOR_TARGET_POS]
        return new_state, reward - penalty, done


class DQN:
    def __init__(self, input_shape, output_size, replay_money_size):
        self.REPLAY_MIN_MEMORY_SIZE = 64
        self.BATCH_SIZE = 32

        self.input_shape = input_shape
        self.output_size = output_size
        self.replay_memory_size = replay_money_size

        self.working_model = self.create_model(self.input_shape)
        self.target_model = self.create_model(self.input_shape)
        self.target_model.set_weights(self.working_model.get_weights())

        self.replay_memory = deque(maxlen=self.replay_money_size)
        self.target_update_counter = 0
        
    def create_model(self):
        model = Sequential()
        model.add(Dense(100, activation='linear', input_shape=self.input_shape))
        model.add(Dense(100, activation='relu'))
        model.add(Flatten())
        model.add(Dense(self.output_size, activation='relu'))
        model.compile(optimizer='adam', loss='mse', metrics=['mse'])
        return model

    def add_replay(replay):
        self.replay_memory.append(replay)

    # def predict(self, state):
    #     return self.working_model.predict(state)

    def train(self):
        if (len(self.replay_memory < self.REPLAY_MIN_MEMORY_SIZE)):
            return

        batch = random.sample(self.replay_memory, self.BATCH_SIZE)

        batch_sample = batch[0]

        current_state_list = np.zeros((self.BATCH_SIZE, batch_sample[0].shape))
        future_state_list = np.zeros((self.BATCH_SIZE, batch_sample[3].shape))

        current_q_list = np.zeros((batch, batch_sample[1].shape))
        future_q_list = np.zeros((batch, batch_sample[1].shape))

        new
        

        for i, (current_state, action, reward, future_state, done) in enumerate(batch):
            current_state_list[i] = current_state
            future_state_list[i] = future_state

        current_q_list = self.working_model.predict(current_state_list)
        future_q_list = self.target_model.predict(current_state_list)





        

    

    