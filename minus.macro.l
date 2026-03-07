MACRODEF !{DIFF} <- {A} minus {B}

USEMACRO copy
USEMACRO goto
USEMACRO noop

#         !Z0 <- copy {A}
#         !Z1 <- copy {B}
# [A]     IF Z1 != 0 GOTO [B]
#         !goto [END]
# [B]     IF Z0 != 0 GOTO [C]
#         !goto [END]
# [C]     DECREMENT Z1
#         DECREMENT Z0
#         !goto [A]
# [END]   !{DIFF} <- copy Z0

        !{DIFF} <- copy {A}
        !Z0 <- copy {B}
[A]     IF Z0 != 0 GOTO [B]
        !goto [END]
[B]     IF {DIFF} != 0 GOTO [C]
        !goto [END]
[C]     DECREMENT Z0
        DECREMENT {DIFF}
        !goto [A]
[END]   !noop