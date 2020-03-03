import turtle as t
import time
import sys


def execString(s, d=100, a=90):
    # Window properties
    t.bgcolor("black")
    t.color("white")

    # Drawing
    t.begin_fill()
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
        else:
            pass
    time.sleep(3)
    t.end_fill()


def main():
    # Arguments
    if len(sys.argv) < 2:
        print("Expecting argument.")
        exit(1)

    filename = sys.argv[1]
    f = open(filename, "r")
    s = f.read()
    execString(s, d=50)
    f.close()


if __name__ == "__main__":
    main()
