import pygame
import random
from math import floor
import copy
from chesspieces import *
from ai import *
import time
from matplotlib import pyplot as plt
import json

def main():
    selected = None
    reachables = []
    need_to_draw = True # Should the screen be draw next frame?
    last_move = () # The last move the ai made to draw it (startx, starty, endx, endy)
    pygame.init()
    screen = Screen()
    field = Field()
    mouse = Mouse()
    ithmove = 0 # Number of moves so far
    isWhitesTurn = True # is it the white's players turn
    # player1 is always white
    player1 = 1
    player2 = RustAI(-1, 4)
    #player1 = AI(1, 2)
    #player2 = -1
    currentPlayer = player1

    timeTaken = []

    frame = 0
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                plt.plot(timeTaken)
                plt.ylabel('Time taken in seconds')
                plt.show()
                running = False
                quit()
            if event.type == pygame.MOUSEBUTTONDOWN:
                pos = pygame.mouse.get_pos()
                mouse.x = floor(pos[0]/screen.cell_size)
                mouse.y = floor(pos[1]/screen.cell_size)
                mouse.clicked = True

        if mouse.clicked and isinstance(currentPlayer, int):
            # Player's turn
            need_to_draw = True
            clicked = field.get(mouse.x, mouse.y)
            if clicked is not None and clicked.color is currentPlayer:
                # The player selected one of his peces
                selected = mouse.getPos()
                reachables = field.get(selected[0], selected[1]).check_reachables(field, selected[0], selected[1])
                # Check if any of the moves in [reachables] will result in a Checkmate
                reachables_copy = copy.deepcopy(reachables)
                reachables = []
                # Loop through [reachables_copy] and add all viable moves(not in check moves) to [reachables]
                for i, reachable in enumerate(reachables_copy):
                    move = (selected[0], selected[1], reachable [0], reachable[1])
                    think_field = copy.deepcopy(field).move(move)
                    if think_field.is_checkmate(currentPlayer):
                        # This move results in a check, so don't add
                        pass
                    else:
                        reachables.append(reachable)
            elif selected is not None and mouse.getPos() in reachables:
                # The player clicked on a reachable field, so move that piece
                field.move((selected[0], selected[1], mouse.getPos()[0], mouse.getPos()[1]))
                selected = None
                reachables = []
                last_move = ()
                isWhitesTurn = not isWhitesTurn
                currentPlayer = player1 if isWhitesTurn else player2
            else:
                selected = None
                reachables = []
            if selected == clicked:
                selected = None
                reachables = []

        elif isinstance(currentPlayer, PythonAI) or isinstance(currentPlayer, RustAI):
            # The AI's turn
            start_time = time.time()
            last_move = currentPlayer.make_move(field)
            print("[%i]" % ithmove, round(time.time()-start_time,3), "Seconds taken by", "white" if isWhitesTurn else "black")
            ithmove += 1
            if last_move is None:
                print("----------------------")
                print("Gameover: %s cannot move" % ("White" if currentPlayer.color == 1 else "Black"))
                time.sleep(1000)
            start = field.get(last_move[0], last_move[1])
            # Validate the ai's move
            if start is None or not (last_move[2], last_move[3]) in\
                start.check_reachables(field, last_move[0], last_move[1]):
                print("Invalid move by AI:", start, last_move)

            field.move(last_move)
            isWhitesTurn = not isWhitesTurn
            currentPlayer = player1 if isWhitesTurn else player2
            need_to_draw = True
            timeTaken.append(round(time.time()-start_time,3))

        # DRAW
        if need_to_draw:
            need_to_draw = False
            screen.draw_grid()
            if last_move:
                screen.draw_circle(last_move[0], last_move[1], (255,165,0))
                screen.draw_circle(last_move[2], last_move[3], (255,165,0))
            if selected is not None:
                screen.draw_circle(selected[0], selected[1], (0,255,0))
                for point in reachables:
                    screen.draw_circle(point[0], point[1], (255,255,0))
            field.draw_all(screen)

            frame += 1
            mouse.reset()
            pygame.display.update()


class Screen:
    def __init__(self):
        self.cell_size = 80
        self.screen = pygame.display.set_mode((8*self.cell_size, 8*self.cell_size))

    def draw_rect(self, x, y, width, height, color):
        pygame.draw.rect(self.screen, color, [x*self.cell_size, y*self.cell_size, width*self.cell_size, height*self.cell_size])

    def draw_circle(self, x, y, color):
        pygame.draw.circle(self.screen, color, (int(x*self.cell_size+self.cell_size/2), int(y*self.cell_size+self.cell_size/2)), int(self.cell_size/2))

    def draw_image(self, image_src, x, y):
        image = pygame.image.load(image_src)
        self.screen.blit(pygame.transform.scale(image, (self.cell_size, self.cell_size)), (x*self.cell_size, y*self.cell_size))

    def draw_grid(self):
        for x in range(8):
            for y in range(8):
                if (x+y)%2 == 0:
                    color = (255,)*3
                else:
                    color = (100,)*3
                self.draw_rect(x, y, 1, 1, color)
                """myfont = pygame.font.SysFont("Comic Sans MS", 20)
                label = myfont.render(str(x)+","+str(y), 1, (0,0,255))
                self.screen.blit(label, (x*self.cell_size, y*self.cell_size))"""


class Mouse:
    def __init__(self):
        self.clicked = False
        self.x = 0
        self.y = 0

    def getPos(self):
        return (self.x, self.y)

    def reset(self):
        self.clicked = False


class Field:
    def __init__(self):
        self.cells = []
        for x in range(8):
            self.cells.append([])
            for y in range(8):
                self.cells[x].append(None)

        self.set(0, 7, Rook 	( 1))
        self.set(1, 7, Knight 	( 1))
        self.set(2, 7, Bishop 	( 1))
        self.set(3, 7, Queen 	( 1))
        self.set(4, 7, King 	( 1))
        self.set(5, 7, Bishop 	( 1))
        self.set(6, 7, Knight 	( 1))
        self.set(7, 7, Rook 	( 1))
        for x in range(8):
            self.set(x, 6, Pawn ( 1))
        self.set(0, 0, Rook 	(-1))
        self.set(1, 0, Knight	(-1))
        self.set(2, 0, Bishop	(-1))
        self.set(3, 0, Queen 	(-1))
        self.set(4, 0, King 	(-1))
        self.set(5, 0, Bishop 	(-1))
        self.set(6, 0, Knight 	(-1))
        self.set(7, 0, Rook 	(-1))
        for x in range(8):
            self.set(x, 1, Pawn (-1))
        """
        self.set(4,7, King(1))
        self.set(3,7, Bishop(1))
        self.set(5,7, Bishop(1))
        self.set(3,0, Knight(-1))
        self.set(4,0, King(-1))
        self.set(5,0, Knight(-1))

        for x in range(8):
            self.set(x, 7, King(1))
            self.set(x, 0, King(-1))
        self.set(4, 2, King(1))
        self.set(4, 5, King(-1))"""

    def get(self, x, y):
        return self.cells[x][y]

    def set(self, x, y, value):
        self.cells[x][y] = value

    def move(self, move):
        # move = (startX, startY, endX, endY)
        self.set(move[2], move[3], self.get(move[0], move[1]))
        self.set(move[0], move[1], None)
        # Make a Pawn only double jump once
        piece = self.get(move[2], move[3])
        if type(piece) is Pawn:
            piece.has_first_move = False
            # If the pawn is at the end of the field, replace him with a queen
            if (move[3] == 0 and piece.color == 1) or (move[3] == 7 and piece.color ==-1):
                self.set(move[2], move[3], Queen(piece.color))
        return self

    def draw_all(self, screen):
        for x, row in enumerate(self.cells):
            for y, cell in enumerate(row):
                if not cell == None:
                    cell.draw(screen, x, y)

    def get_possible_moves(self, color):
        # Get all moves that color can make
        moves = []
        for x in range(8):
            for y in range(8):
                cell = self.get(x, y)
                if not cell == None:
                    if cell.color == color:
                        reachables = cell.check_reachables(self, x, y)
                        # Add the moves to the possible moves
                        for reachable in reachables:
                            moves.append((x, y, reachable[0], reachable[1]))
        return moves

    def get_score(self, color):
        # Calculate how good that color is doing  len(color) - len(color*-1)
        score = 0
        for x in range(8):
            for y in range(8):
                cell = self.get(x, y)
                if not cell == None:
                    score += cell.color * color * cell.worth
        return score

    def is_checkmate(self, color): # returns if the given color is in check
        # Find the Rook
        for x in range(8):
            for y in range(8):
                cell = self.get(x, y)
                if cell:
                    if type(cell) == King and cell.color == color:
                        rook = cell
                        rookX = x
                        rookY = y
        moves = self.get_possible_moves(color * -1)
        for move in moves:
            if move[2] == rookX and move[3] == rookY:
                # The King is in Check
                return True
        return False

    def toJson(self):
        def figureToJson(fig):
            color = "White" if fig.color == 1 else "Black"
            return { "color": color, "figure": fig.name.title() }
        return json.dumps(self.cells, default=figureToJson)


if __name__ == '__main__':
    main()
