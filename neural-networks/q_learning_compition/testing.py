import gym

env = gym.make('CartPole-v1')
env.reset()

for _ in range(2):
    while(True):
        env.render()
        state, reward, done, _ = env.step(1)

        print(state, reward, done)

        if done:
            env.reset()
            break
    

env.close()