import gym

from stable_baselines3 import PPO
from servo_env_real import Servo_Env_Real

env = Servo_Env_Real()
model = PPO.load("Model_1")

# model.set_env(env)
# model.learning_rate = 0.005
# model.learn(total_timesteps= 1_000, reset_num_timesteps=True)
# model.save("Model_1")

SAMPLES = 100
reward_sum = 0

obs = env.reset()
for i in range(SAMPLES):
    action, _state = model.predict(obs, deterministic=True)
    obs, reward, done, _ = env.step(action)
    reward_sum += reward
    if done:
      obs = env.reset()

print(f"Reward sum = {reward_sum}")
print(f"Average reward = {reward_sum/SAMPLES}")