---
title: Shell Reader
tags: [documentation]
last_updated: March 12, 2021
sidebar: mydoc_sidebar
keywords: shellreader
permalink: mydoc_shellreader.html
---
# Using the shell reader in sl-sh
<hr>

## from the repl
The shell reader will be invoked on any input at the repl that is not
surrounded by parens '(', ')'.  It have an implied $(...) around the input
so their is no need to surround shell commands in boiler plate.

### shell expressions $(...)
This form will expand to one or more function/system call commands.  It
supports the following opperators:
- \|  Pipe, pipe the output from the first command to the second in a chain.
- \|& Pipe with stderr in pipe.  Like above but stderr is also piped with stdout.
- ;  Do multiple commands, the ; splits tokens and runs each set.  Will not
stop on errors.
- && And- like do above but will stop if a function returns nil or a system
command returns non-zero.
- || Or, like do but will stop as soon as one expression is 'true' (non-nil
for an expression and 0 exit code for a system command).

### shell expressions $%(...)

### variable expression $NAME

### variable expression ${NAME}

## Expansions

### bracket {...}

### tilde ~

## string interpolation


[<-- back to the docs]( {{ site.url }} )

