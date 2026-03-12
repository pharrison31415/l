MACRODEF !{TARGET} <- {A} less than {B}

USEMACRO minus
USEMACRO not
USEMACRO noop

# Z0 = B - A
!Z0 <- {B} minus {A}

# TARGET = not(Z0 == 0)
!Z1 <- not Z0
!{TARGET} <- not Z1
