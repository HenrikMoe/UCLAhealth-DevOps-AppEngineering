# Developer notes - Henrik Moe 2/8/24 - 1) After successfully reaching the exit of the maze the maze does not acknowledge     # success and does not reset. 2) After consuming item noting happens

# Diganosis:  1) if maze[player_pos[0]][player_pos[1]] == EXIT:# the first item returns the players icon, the second will     # always be exit. 2) no ideas for the item note

# Solutions: 1) Hardcode the exit coordinates into the if comparison. 2) Write additional logic to store the exits' location. 

import random

# Maze dimensions
MAZE_WIDTH = 10
MAZE_HEIGHT = 10

# Maze symbols
WALL = '#'
EMPTY = ' '
PLAYER = 'P'
EXIT = 'E'
ITEM = '*'

# Function to initialize the maze
def initialize_maze():
    maze = [[EMPTY] * MAZE_WIDTH for _ in range(MAZE_HEIGHT)]
    # Place walls
    for i in range(MAZE_HEIGHT):
        maze[i][0] = WALL
        maze[i][-1] = WALL
    for j in range(MAZE_WIDTH):
        maze[0][j] = WALL
        maze[-1][j] = WALL
    # Place player
    maze[1][1] = PLAYER
    # Place exit
    maze[MAZE_HEIGHT - 2][MAZE_WIDTH - 2] = EXIT
    # Place items
    for _ in range(3):  # Adjust the number of items as needed
        while True:
            x = random.randint(1, MAZE_WIDTH - 2)
            y = random.randint(1, MAZE_HEIGHT - 2)
            if maze[y][x] == EMPTY:
                maze[y][x] = ITEM
                break
    return maze

# Function to print the maze
def print_maze(maze):
    for row in maze:
        print(''.join(row))

# Function to move the player
def move_player(maze, direction):
    player_pos = find_player(maze)
    new_pos = (player_pos[0] + direction[0], player_pos[1] + direction[1])
    if maze[new_pos[0]][new_pos[1]] != WALL:
        maze[player_pos[0]][player_pos[1]] = EMPTY
        maze[new_pos[0]][new_pos[1]] = PLAYER
        return True
    return False

# Function to find the player's position
def find_player(maze):
    for i in range(len(maze)):
        for j in range(len(maze[i])):
            if maze[i][j] == PLAYER:
                return (i, j)

# Main function to run the game
def main():
    maze = initialize_maze()
    print("Welcome to the maze game!")
    print("Use WASD to move. Try to find the exit (E) while collecting items (*)!")
    while True:
        print_maze(maze)
        direction = input("Enter your move (WASD): ").upper() 
        if direction == 'W':
            moved = move_player(maze, (-1, 0))
        elif direction == 'S':
            moved = move_player(maze, (1, 0))
        elif direction == 'A':
            moved = move_player(maze, (0, -1))
        elif direction == 'D':
            moved = move_player(maze, (0, 1))
        else:
            print("Invalid move! Use WASD.")
            continue
        if not moved:
            print("Cannot move there! Try another direction.")
        else:
            print(f"Exit Position: {EXIT}")
            player_pos = find_player(maze)
            vay = maze[player_pos[0]][player_pos[1]]
            print(f"Player Position: {vay}")
            if maze[player_pos[0]][player_pos[1]] == maze[MAZE_HEIGHT - 2][MAZE_WIDTH - 2]:
                print("Congratulations! You found the exit!")
                break

if __name__ == "__main__":
    main()