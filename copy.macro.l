MACRODEF !{TARGET} <- copy {SOURCE}

USEMACRO goto
USEMACRO noop
USEMACRO zero

    !{TARGET} <- ZERO
    !Z0 <- ZERO
[A] IF {SOURCE} != 0 GOTO [B]
    !goto [C]
[B] DECREMENT {SOURCE}
    INCREMENT {TARGET}
    INCREMENT Z0
    !goto [A]
[C] IF Z0 != 0 GOTO [D]
    !goto [END]
[D] DECREMENT Z0
    INCREMENT {SOURCE}
    !goto [C]

[END]   !noop