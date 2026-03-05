USEMACRO goto

# Identity for X0
[A] IF X0 != 0 GOTO [B]
    !goto [C]
[B] DECREMENT X0
    INCREMENT Y
    !goto [A]

# Identity for X1
[C] IF X1 != 0 GOTO [D]
    STOP
[D] DECREMENT X1
    INCREMENT Y
    !goto [C]
