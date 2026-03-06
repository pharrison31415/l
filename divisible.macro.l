MACRODEF !{TARGET} <- {N} divisible {D}

USEMACRO copy
USEMACRO minus
USEMACRO eq
USEMACRO not
USEMACRO and
USEMACRO goto
USEMACRO zero
USEMACRO noop

        # result defaults to false
        !{TARGET} <- ZERO

        # Z0 will hold the running remainder
        !Z0 <- copy {N}

        # Z1 <- Z0 - D
[LOOP]  !Z1 <- Z0 minus {D}

        # if Z1 == 0, then either Z0 == D or Z0 < D
        !Z2 <- not Z1
        IF Z2 != 0 GOTO [CHECK]

        # otherwise keep going
        !Z0 <- copy Z1
        !goto [LOOP]

        # Check for zero
[CHECK] !Z3 <- Z0 eq {D}
        IF Z3 != 0 GOTO [DIV]
        !goto [NOT]

        # divisible → return true
[DIV]   INCREMENT {TARGET}
        !goto [END]

        # not divisible, leave TARGET = 0
[NOT]   !goto [END]


[END]   !noop
