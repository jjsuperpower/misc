# Copyright 2018 Tensorforce Team. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ==============================================================================

import os
import logging

import tensorflow as tf

from tensorforce.agents import Agent
from tensorforce.environments import Environment
from tensorforce.execution import Runner


os.environ['TF_CPP_MIN_LOG_LEVEL'] = '3'
logger = tf.get_logger()
logger.setLevel(logging.ERROR)


def main():
    # Create an OpenAI-Gym environment
    environment = Environment.create(environment='gym', level='CartPole-v1')

    # Create a PPO agent
    agent = Agent.create(
        agent='dqn', environment=environment,
        # memory=100,
        # # Optimization
        # batch_size=10, update_frequency=2, learning_rate=1e-3,

        summarizer=dict(
            directory='data/summaries',
            # list of labels, or 'all'
            labels=['graph', 'entropy', 'kl-divergence', 'losses', 'rewards'],
            frequency=100  # store values every 100 timesteps
            # (infrequent update summaries every update; other configurations possible)
            ),
        recorder=None
    )

    # Initialize the runner
    runner = Runner(agent=agent, environment=environment)

    # Start the runner
    runner.run(num_episodes=10000)
    runner.close()


if __name__ == '__main__':
    main()