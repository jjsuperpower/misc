import gym

from stable_baselines3 import PPO
from servo_env_sim import Servo_Env_Sim

env = Servo_Env_Sim()
model = PPO.load("Model_2")

SAMPLES = 1000
reward_sum = 0

obs = env.reset()
for i in range(SAMPLES):
    action, _state = model.predict(obs, deterministic=True)
    obs, reward, done, info = env.step(action)
    reward_sum += reward
    if done:
      obs = env.reset()

print(f"Reward sum = {reward_sum}")
print(f"Average reward = {reward_sum/SAMPLES}")