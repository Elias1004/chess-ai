import subprocess
import json
import time

class RustAI:
    def __init__(self, color, look_ahead):
        self.color = color
        color_string = "white" if color == 1 else "black"
        self.max_look_ahead = look_ahead
        self.process = subprocess.Popen(["../target/release/chess-ai", color_string, str(look_ahead)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, encoding="utf8")


    def make_move(self, field):
        #print("Starting Score:", field.get_score(self.color))
        return self.get_best_score(field, self.max_look_ahead)

    def get_best_score(self, field, look_ahead): # Return's the value of the highest possible score
        # Write state of the field to the subprocess' stdin
        self.process.stdin.write(field.toJson() + "\n")
        self.process.stdin.flush()
        # Read out the calculated move from stdout
        output = self.process.stdout.readline()
        try:
            move = json.loads(output)
        except:
            print("AI errored")
            time.sleep(1000)
        if move is None:
            return None
        return (move["from"][0], move["from"][1], move["to"][0], move["to"][1])

from copy import deepcopy
import random

class PythonAI:
    def __init__(self, color, look_ahead):
        self.color = color
        self.max_look_ahead = look_ahead

    def make_move(self, field):
        #print("Starting Score:", field.get_score(self.color))
        return self.get_best_score(field, self.max_look_ahead)

    def get_best_score(self, field, look_ahead): # Return's the value of the highest possible score
        if look_ahead == 0:
            # This is the lowest point in the chain of recursion
            return field.get_score(self.color)
        scores = []
        # Every other move the moves of the enemy player needs to be simulated
        perspective = 1 if (self.max_look_ahead-look_ahead)%2==0 else -1
        moves = field.get_possible_moves(self.color * perspective)
        for n, move in enumerate(moves):
            score = self.get_best_score(deepcopy(field).move(move), look_ahead-1)
            scores.append(score)
            # Debug: Print the progress
            #if look_ahead == self.max_look_ahead:
                #print(str(n+1) + "/" + str(len(moves)), "\tScore: ", score, "\tMove:", move)

        # If this function is the toppest function call return the move to make
        if look_ahead == self.max_look_ahead:
            # Get the indices of all the values that share the same highest value
            highest_score = max(scores)
            highest_moves = [move for move, score in zip(moves, scores) if score == highest_score]
            return random.choice(highest_moves)
        # Otherwise return the highest calculated value for the function calls above to process
        if perspective == -1:
            # Since the opposite player chooses the worst outcome for the AI, 
            # return the lowest of the values, if it is the opposite player's turn
            if len(scores) == 0:
                # In this field there are no possible moves to make, which will be treated as
                # winning, so assign the highest possible score
                return 10000
            return min(scores)
        if len(scores) == 0:
            # Worst Case Scenario; no more possible moves for self; assign bad score
            return -10000
        return max(scores)
