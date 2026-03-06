MACRODEF !{TARGET} <- {A} and {B}

USEMACRO zero
USEMACRO goto
USEMACRO noop

            !{TARGET} <- ZERO
            IF {A} != 0 GOTO [CHECKB]
            !goto [END]

[CHECKB]    IF {B} != 0 GOTO [SET]
            !goto [END]

[SET]       INCREMENT {TARGET}

[END]       !noop
