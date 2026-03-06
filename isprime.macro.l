MACRODEF !{TARGET} <- isprime {N}

USEMACRO copy
USEMACRO eq
USEMACRO divisible
USEMACRO goto
USEMACRO noop
USEMACRO zero

        !{TARGET} <- ZERO
        !Z0 <- copy {N}

        # if N == 0, return 0
        !Z1 <- ZERO
        !Z2 <- Z0 eq Z1
        IF Z2 != 0 GOTO [END]

        # if N == 1, return 0
        INCREMENT Z1
        !Z2 <- Z0 eq Z1
        IF Z2 != 0 GOTO [END]

        # divisor = 2
        INCREMENT Z1
        !Z3 <- copy Z1

[LOOP]  !Z4 <- Z3 eq Z0
        IF Z4 != 0 GOTO [PRIME]

        !Z5 <- Z0 divisible Z3
        IF Z5 != 0 GOTO [END]

        INCREMENT Z3
        !goto [LOOP]

[PRIME] INCREMENT {TARGET}

[END]   !noop
