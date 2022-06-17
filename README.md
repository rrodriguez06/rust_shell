# rust_shell
A simple tui shell writen in rust. It's a training project.

The shell can handle simple commands (like cd, ls, ...) and update it's output while commands are runing.

Still need a lot of optimization and work in general.

# Usage
Run the program then follow bindings writen on the program screen.
If you want the list of all bindings and custom builtin commands just run "help" builtin command in insert mode.

# Bindings
## in Normal mode
I -> enter insert mode

Q -> exit the program

## in Insert mode
Enter -> launch the command

Tab -> enter Completion mode

Down -> enter History mode

Esc -> exit Insert mode

## in Completion mode
Tab -> select the completion pattern

Enter -> input selected completion pattern

Esc -> exit Completion mode

## in History mode
Tab -> select the history command

Enter -> input selected history command

Esc -> exit History mode

## in Output mode (if stuck in it)
Esc -> exit output mode

# Custom commands
help -> display helping popup

c -> clear the output section
