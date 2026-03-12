MACRODEF !{TARGET} <- prime after {N}

USEMACRO copy
USEMACRO isprime
USEMACRO lt
USEMACRO divisible
USEMACRO goto
USEMACRO noop
USEMACRO zero

        !Z0 <- ZERO
        
        # Z0 = 2
        INCREMENT Z0
        INCREMENT Z0

        # if X0 < 2, answer is 2
        !Z1 <- X0 less than Z0
        IF Z1 != 0 GOTO [RET2]

        # candidate = X0 + 1
        !Z2 <- copy X0
        INCREMENT Z2

        # if candidate is even, make it odd
        # special case 2 is already handled above, so this is safe
        !Z3 <- Z2 divisible Z0    # Z0 is still 2
        IF Z3 != 0 GOTO [MKODD]
        !goto [LOOP]

[MKODD] INCREMENT Z2

[LOOP]  !Z4 <- isprime Z2
        IF Z4 != 0 GOTO [FOUND]

        # skip even numbers: only test odd candidates
        INCREMENT Z2
        INCREMENT Z2
        !goto [LOOP]

[FOUND] !Y <- copy Z2
        !goto [END]

[RET2]  !Y <- ZERO
        INCREMENT Y
        INCREMENT Y

[END]   !noop
