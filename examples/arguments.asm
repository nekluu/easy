# ARGUMENTS EXAMPLE
# FOR LINUX.
section bss 
    arguments reb 1024

section data
    args ascii "/proc/self/cmdline" 0
    return ascii 10

section code 
getargs:
    # open
    mov args, r0
    mov 0, r1 
    syscall 2

    # read
    mov r6, r0 
    mov arguments, r1 
    mov 1024, r2 
    syscall 0
    mov r6, r11

    # write
    mov 1, r0 
    mov arguments, r1 
    mov 1024, r2 
    syscall 1

    # just \n
    mov 1, r0
    mov return, r1 
    mov 1, r2
    syscall 1
