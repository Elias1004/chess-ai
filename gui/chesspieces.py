import pygame

class Chesspiece:
    def __init__(self, name, color):
        self.color = color
        self.name = name
        self.load_image(name, color)

    def load_image(self, name, color):
        color = ["black",0,"white"][color+1]

        self.image_src = "sprites/%s_%s.png" %(name, color)

    def draw(self, screen, x, y):
        screen.draw_image(self.image_src, x, y)

    def check_reachable_cell(self, field, x, y):
        # Check if the cell is outside the field
        if x < 0 or x > 7 or y < 0 or y > 7:
            return None
        if field.get(x, y) is None: # Is that cell free? Then add it to [reachables]
            return True
        else:
            # If there is a chesspiece  on that cell then end, if that has the opposite color add that before
            if field.get(x, y).color is not self.color:
                return False
            return None

    def __str__(self):
        return self.name + str(self.color)

class Rook(Chesspiece):
    def __init__(self, color):
        Chesspiece.__init__(self, "rook", color)
        self.worth = 5

    def check_reachables(self, field, selfx, selfy):
        reachables = []
        # Check four directions
        for dir in range(4):
            for d in range(1, 8):
                if dir == 0:
                    # Left
                    x = selfx - d
                    y = selfy
                if dir == 1:
                    # Right
                    x = selfx + d
                    y = selfy
                if dir == 2:
                    # Up
                    x = selfx
                    y = selfy - d
                if dir == 3:
                    # Down
                    x = selfx
                    y = selfy + d

                out = self.check_reachable_cell(field, x, y)
                if out is True:
                    reachables.append((x, y))
                else:
                    if out is False:
                        reachables.append((x, y))
                    break
        return reachables


class Bishop(Chesspiece):
    def __init__(self, color):
        Chesspiece.__init__(self, "bishop", color)
        self.worth = 3

    def check_reachables(self, field, selfx, selfy):
        reachables = []
        # Check four directions
        for dir in range(4):
            for d in range(1, 8):
                if dir == 0:
                    x = selfx - d
                    y = selfy - d
                if dir == 1:
                    x = selfx + d
                    y = selfy + d
                if dir == 2:
                    x = selfx + d
                    y = selfy - d
                if dir == 3:
                    x = selfx - d
                    y = selfy + d

                out = self.check_reachable_cell(field, x, y)
                if out is True:
                    reachables.append((x, y))
                else:
                    if out is False:
                        reachables.append((x, y))
                    break
        return reachables


class Queen(Chesspiece):
    def __init__(self, color):
        Chesspiece.__init__(self, "queen", color)
        self.worth = 9

    def check_reachables(self, field, selfx, selfy):
        reachables = []
        # Check eight directions
        for dir in range(8):
            for d in range(1, 8):
                if dir == 0:
                    x = selfx - d
                    y = selfy - d
                if dir == 1:
                    x = selfx + d
                    y = selfy + d
                if dir == 2:
                    x = selfx + d
                    y = selfy - d
                if dir == 3:
                    x = selfx - d
                    y = selfy + d
                if dir == 4:
                    x = selfx - d
                    y = selfy
                if dir == 5:
                    x = selfx + d
                    y = selfy
                if dir == 6:
                    x = selfx
                    y = selfy - d
                if dir == 7:
                    x = selfx
                    y = selfy + d

                out = self.check_reachable_cell(field, x, y)
                if out is True:
                    reachables.append((x, y))
                else:
                    if out is False:
                        reachables.append((x, y))
                    break
        return reachables


class Knight(Chesspiece):
    def __init__(self, color):
        Chesspiece.__init__(self, "knight", color)
        self.worth = 3

    def check_reachables(self, field, selfx, selfy):
        reachables = []
        for dx in [-2,-1,1,2]:
            for dy in [-2,-1,1,2]:
                if abs(dx) is not abs(dy):
                    x = selfx+dx
                    y = selfy+dy
                    if x >= 0 and x <= 7 and y >= 0 and y <= 7:
                        if field.get(x, y) == None:
                            reachables.append((x, y))
                        elif field.get(x, y).color is not self.color:
                            reachables.append((x, y))

        return reachables


class King(Chesspiece):
    def __init__(self, color):
        Chesspiece.__init__(self, "king", color)
        self.worth = 1000

    def check_reachables(self, field, selfx, selfy):
        reachables = []
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                x = selfx + dx
                y = selfy + dy
                if x >= 0 and x <= 7 and y >= 0 and y <= 7:
                    if field.get(x, y) is None:
                        reachables.append((x, y))
                    elif field.get(x, y).color is not self.color:
                        reachables.append((x, y))

        return reachables


class Pawn(Chesspiece):
    def __init__(self, color):
        Chesspiece.__init__(self, "pawn", color)
        self.has_first_move = True
        self.worth = 1

    def check_reachables(self, field, selfx, selfy):
        reachables = []
        # Forward movement
        dx = 0
        dy = -1 * self.color
        x = selfx + dx
        y = selfy + dy
        if x >= 0 and x <= 7 and y >= 0 and y <= 7:
            if field.get(x, y) is None:
                reachables.append((x, y))
        # Double move on first move
        if self.has_first_move:
            dx = 0
            dy = -2 * self.color
            x = selfx + dx
            y = selfy + dy
            if x >= 0 and x <= 7 and y >= 0 and y <= 7:
                if field.get(x, y) is None and field.get(x, y+self.color) is None:
                    reachables.append((x, y))

        # Sideway/Hit Movement
        for dx in [-1,1]:
            dy = -1 * self.color
            x = selfx + dx
            y = selfy + dy
            if x >= 0 and x <= 7 and y >= 0 and y <= 7:
                if field.get(x, y) is not None:
                    if field.get(x, y).color is not self.color:
                        reachables.append((x, y))

        return reachables
