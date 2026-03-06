MACRODEF !{SUM} <- {A0} plus {A1}

USEMACRO copy
USEMACRO goto
USEMACRO noop

        !{SUM} <- copy {A0}
        !Z0 <- copy {A1}
[A]     IF Z0 != 0 GOTO [B]
        !goto [END]
[B]     DECREMENT Z0
        INCREMENT {SUM}
        !goto [A]
[END]   !noop