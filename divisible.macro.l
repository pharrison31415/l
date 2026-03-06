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

        # check if subtraction changed anything
        # if Z1 == Z0 then subtraction failed (D > remainder)
        !Z2 <- Z1 eq Z0
        IF Z2 != 0 GOTO [NOT]

        # if Z1 == 0 then we hit exact multiple
        !Z3 <- not Z1
        IF Z3 != 0 GOTO [DIV]

        # otherwise continue subtracting
        !Z0 <- copy Z1
        !goto [LOOP]

        # divisible → return true
[DIV]   INCREMENT {TARGET}
        !goto [END]

        # not divisible, leave TARGET = 0
[NOT]   !goto [END]


[END]   !noop
