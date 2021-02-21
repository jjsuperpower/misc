from mpu6050 import mpu6050
import RPi.GPIO as GPIO

import math
from time import sleep, time
import numpy as np

SENSOR_ACCEL_MIN = -9
SENSOR_ACCEL_MAX = 9
SENSOR_GYRO_MIN  = -4
SENSOR_GYRO_MAX = 4
ACCEL_SAMPLE_SIZE = 32
GYRO_SAMPLE_SIZE = 32
ACCEL_STATES     = 10
GYRO_STATES      = 1

DISCRETE_ACCEL_RANGE = (SENSOR_ACCEL_MAX - SENSOR_ACCEL_MIN) / ACCEL_STATES
DISCRETE_GYRO_RANGE = (SENSOR_GYRO_MAX - SENSOR_GYRO_MIN) / GYRO_STATES

SERVO_PIN = 22
SERVO_FREQ = 50
SERVO_DUTY_MIN = 3
SERVO_DUTY_MAX = 11
SERVO_TIME_DELAY = 1
SERVO_ACTIONS = 16

DISCRETE_ACTIONS_RANGE = (SERVO_DUTY_MAX - SERVO_DUTY_MIN) / SERVO_ACTIONS

ACTION_REWARD_FACTOR = 100
TIME_PEN_FACTOR = 0
MPU_TOLERANCE = 2

LEARNING_RATE = .5
DISCOUNT = .8
EPISODES = 1000
EPSILON_START = .05
EPSILON_END = .01

EPISLON_DECAY_RATE = (EPSILON_START - EPSILON_END) / EPISODES


class Enviroment:
    def __init__(self):

        GPIO.setwarnings(False)
        GPIO.setmode(GPIO.BOARD)
        GPIO.setup(SERVO_PIN, GPIO.OUT)

        self.servo = GPIO.PWM(SERVO_PIN, SERVO_FREQ)
        self.sensor = mpu6050(0x68)

        self.target = 0
        self.servo_pos = 0
        self.time_start = 0
        self.time_stop = 0
        self.reward = 0
        self.penalty = 0

    def reset(self):
        self.servo_pos = np.random.uniform(SERVO_DUTY_MIN, SERVO_DUTY_MAX)
        self.servo.start(self.servo_pos)
        
        self.target = 0

        sleep(SERVO_TIME_DELAY)
        self.time_start = time()

    def stop(self):
        self.servo.stop()

    def get_accel_avg(self):
        accel_list = []
        for i in range(ACCEL_SAMPLE_SIZE):
            accel_list.append(self.sensor.get_accel_data()['y'])
        return sum(accel_list) / len(accel_list)
        
    def get_gyro_avg(self):
        gyro_list = []
        for i in range(GYRO_SAMPLE_SIZE):
            gyro_list.append(self.sensor.get_gyro_data()['y'])
        return sum(gyro_list) / len(gyro_list)


    def get_descrete_state(self):

        def get_accel_descrete_state(accel_state):
            discrete_accel_state = (accel_state - SENSOR_ACCEL_MIN) / DISCRETE_ACCEL_RANGE - 1
            return int(discrete_accel_state)

        def get_gyro_descrete_state(gyro_state):
            discrete_gyro_state = (gyro_state - SENSOR_GYRO_MIN) / DISCRETE_GYRO_RANGE - 1
            return int(discrete_gyro_state)

        accel_state = get_accel_descrete_state(self.get_accel_avg())
        gyro_state = get_gyro_descrete_state(self.get_gyro_avg())
        state = [accel_state, gyro_state]

        return state


    def step(self, action):

        self.servo.ChangeDutyCycle(action * DISCRETE_ACTIONS_RANGE + SERVO_DUTY_MIN)
        sleep(SERVO_TIME_DELAY)

        self.reward = (MPU_TOLERANCE / abs(self.target - self.get_accel_avg())) * ACTION_REWARD_FACTOR

        if (self.reward >= ACTION_REWARD_FACTOR):
            self.time_stop = time()
            time_dif = (self.time_stop - self.time_start) * TIME_PEN_FACTOR

            self.done = True
            self.penalty = time_dif

        else:
            self.done = False

        print('Reward = ' + str(int(self.reward)))

        new_state = self.get_descrete_state()
        return new_state, self.reward - self.penalty, self.done
        

#q_table = np.random.uniform(low=0, high=ACTION_REWARD_FACTOR, size=(ACCEL_STATES, GYRO_STATES, SERVO_ACTIONS))
q_table = np.load('q_table.npy')
epsilon = 1
env = Enviroment()


for episode in range(EPISODES):

    env.reset()
    state = env.get_descrete_state()
    done = False

    while not done:
        
        if (np.random.random() > epsilon):
            action = np.argmax(q_table[tuple(state)])
        else:
            action = np.random.randint(low=0, high=SERVO_ACTIONS)

        
        q_index = tuple(state + [action])
        new_state, reward, done = env.step(action)

        # print('q_index is ' + str(q_index))
        # print('q_table[state].shape is ' + str(q_table[tuple(state)].shape))
        # print('action is ' + str(action))
        
        if not done:
            max_future_q = np.max(q_table[tuple(new_state)])
            current_q = q_table[q_index]
            
            new_q = (1 - LEARNING_RATE) * current_q + LEARNING_RATE * (reward + DISCOUNT * max_future_q)
            q_table[q_index] = new_q
            
        else:
            q_table[q_index] = reward

        state = new_state

    if (EPSILON_END < epsilon <= EPSILON_START):
        epsilon -= EPISLON_DECAY_RATE

    if((episode + 1) % 10 == 0):
        np.save('q_table', q_table)
        print('Saving File')
    env.reset()


    print('Finished episode ', str(episode + 1))






