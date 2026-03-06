MACRODEF !{PROD} <- {M0} times {M1}

USEMACRO add
USEMACRO copy
USEMACRO goto
USEMACRO noop
USEMACRO zero

        !Z0 <- ZERO
        !Z1 <- ZERO
        IF X0 != 0 GOTO [C]
        !goto [END]
[C]     !Z1 <- copy {M1}
[A]     IF Z1 != 0 GOTO [B]
        !goto [END]
[B]     DECREMENT Z1
        !Z0 <- add X0 {PROD}
        !{PROD} <- copy Z0
        !goto [A]
[END]   !noop