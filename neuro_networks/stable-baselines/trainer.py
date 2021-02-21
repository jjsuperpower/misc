import gym

from stable_baselines3 import PPO
from servo_env_sim import Servo_Env_Sim

#env = gym.make('MountainCarContinuous-v0')
env = Servo_Env_Sim()

model = PPO('MlpPolicy', env, verbose=1)
model.learn(total_timesteps= 50_000)

model.save("Model_1")

# obs = env.reset()
# for i in range(1000):
#     action, _state = model.predict(obs, deterministic=False)
#     obs, reward, done, info = env.step(action)
#     if done:
#       obs = env.reset()