# Identity for X0
[A] IF X0 != 0 GOTO B
    GOTO C
[B] DECREMENT X0
    INCREMENT Y
    GOTO A

# Identity for X1
[C] IF X1 != 0 GOTO D
    STOP
[D] DECREMENT X1
    INCREMENT Y
    GOTO C