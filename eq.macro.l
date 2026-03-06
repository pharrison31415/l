MACRODEF !{TARGET} <- {A} eq {B}

USEMACRO minus
USEMACRO not
USEMACRO and
USEMACRO noop

!Z0 <- {A} minus {B}
!Z1 <- {B} minus {A}

!Z2 <- not Z0
!Z3 <- not Z1

!{TARGET} <- Z2 and Z3

!noop
