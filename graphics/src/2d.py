import turtle as t
import time
import sys

class TurtleState:
    '''
    Takes a position (x, y) and a heading (h)
    '''
    def __init__(self, x, y, h):
        self.x = x
        self.y = y
        self.h = h

def execString(s, d=100, a=90):
    # Stack for branches
    stack = []

    # Window properties
    t.bgcolor("black")
    t.color("white")
    t.speed(0)

    # Drawing
    time.sleep(1)

    for c in s:
        if c == "F":
            t.forward(d)
        elif c == "f":
            t.penup()
            t.forward(d)
            t.pendown()
        elif c == "+":
            t.left(a)
        elif c == "-":
            t.right(a)
        elif c == "[":
            stack.append(TurtleState(t.xcor(), t.ycor(), t.heading()))
        elif c == "]":
            state = stack.pop()
            t.penup()
            t.setx(state.x)
            t.sety(state.y)
            t.seth(state.h)
            t.pendown()
        elif c == "|":
            t.right(180)
        elif c == "{":
            t.begin_fill()
        elif c == "}":
            t.end_fill()
        else:
            pass
    time.sleep(3)


def main():
    # Arguments
    if len(sys.argv) < 2:
        print("Expecting argument.")
        exit(1)

    d = 10
    a = 22.5
    if len(sys.argv) == 3:
        a = int(sys.argv[2])
    filename = sys.argv[1]
    f = open(filename, "r")
    s = f.read()
    execString(s, d, a)
    f.close()


if __name__ == "__main__":
    main()
