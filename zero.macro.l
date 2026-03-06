MACRODEF !{REG} <- ZERO

USEMACRO noop

    !noop
[A] DECREMENT {REG}
    IF {REG} != 0 GOTO [A]
