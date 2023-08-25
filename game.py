import numpy as np
import win32pipe
import win32file
import time

num_rows = 20
num_cols = 10
goal_position = (19, 9)

# Algoritmo Q-learning
num_states = num_rows * num_cols
num_actions = 4
learning_rate = 0.1
discount_factor = 0.9
num_episodes = 1000

Q = np.zeros((num_states, num_actions))

# Função para mapear uma posição da matriz para um estado
def map_position_to_state(position):
    return position[1] * num_rows + position[0]

# Função para escolher uma ação usando a política epsilon-greedy
def choose_action(state, epsilon):
    if np.random.uniform(0, 1) < epsilon:
        return np.random.choice(num_actions)
    else:
        return np.argmax(Q[state, :])



pipe = win32file.CreateFile(
    r'\\.\pipe\my_bidirectional_pipe',  # Nome do pipe
    win32file.GENERIC_READ | win32file.GENERIC_WRITE,
    0, None, win32file.OPEN_EXISTING, 0, None
)

actions = {
    0:"l",
    1:"d",
    2:"r",
    3:"u"
}

# Loop de treinamento
for episode in range(num_episodes):
    win32file.WriteFile(pipe, "reset".encode())
    win32file.ReadFile(pipe, 1024)[1]
    state = 0
    done = False

    while not done:
        
        action = choose_action(state, epsilon=0.1)
        
        action_ = actions[action]

        win32file.WriteFile(pipe, action_.encode())
        buffer = win32file.ReadFile(pipe, 1024)[1]
        next_state, reward = buffer.decode('utf-8').strip().split(":")
        next_state = int(next_state)
        reward = int(reward[:-1])

        # Atualizando Q
        Q[state, action] = (1 - learning_rate) * Q[state, action] + learning_rate * (
            reward + discount_factor * np.max(Q[next_state, :])
        )

        state = next_state

        if state == map_position_to_state(goal_position):
            done = True

        # input('->')

print(Q)
player_position = (0, 0)
win32file.WriteFile(pipe, "reset".encode())
input("Jogar ?")
while player_position != goal_position:
    action = np.argmax(Q[map_position_to_state(player_position), :])

    action_ = actions[action]
    win32file.WriteFile(pipe, action_.encode())
    win32file.ReadFile(pipe, 1024)[1]

    if action == 3 and player_position[0] > 0:
        player_position = (player_position[0], player_position[1] + 1)
    elif action == 1 and player_position[0] < num_cols - 1:
        player_position = (player_position[0], player_position[1] + 1)
    elif action == 0 and player_position[1] > 0:
        player_position = (player_position[0] - 1, player_position[1] )
    elif action == 2 and player_position[1] < num_rows - 1:
        player_position = (player_position[0] + 1, player_position[1])

    print("Jogador moveu para:", player_position)

    time.sleep(0.5)

print("Jogador venceu! Parabéns!")

input("Sair ?")
win32file.CloseHandle(pipe)