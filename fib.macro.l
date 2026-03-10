MACRODEF !{TARGET} <- fib {N}

USEMACRO copy
USEMACRO plus
USEMACRO minus
USEMACRO eq
USEMACRO goto
USEMACRO zero
USEMACRO noop

        # if n == 0: return 0
        !Z4 <- {N} eq Z5
        IF Z4 != 0 GOTO [ZERO]

        # if n == 1: return 1
        INCREMENT Z5
        !Z4 <- {N} eq Z5
        IF Z4 != 0 GOTO [ONE]

        # a = 0
        !Z0 <- ZERO

        # b = 1
        !Z1 <- ZERO
        INCREMENT Z1

        # i = 1
        !Z2 <- copy Z1

        # if i == n: done
[LOOP]  !Z4 <- Z2 eq {N}
        IF Z4 != 0 GOTO [DONE]

        # c = a + b
        !Z3 <- Z0 plus Z1

        # a = b
        !Z0 <- copy Z1

        # b = c
        !Z1 <- copy Z3

        # increment i
        INCREMENT Z2

        !goto [LOOP]

[DONE]  !{TARGET} <- copy Z1
        !goto [END]

[ZERO]  !{TARGET} <- ZERO
        !goto [END]

[ONE]   !{TARGET} <- ZERO
        INCREMENT {TARGET}

[END]   !noop
