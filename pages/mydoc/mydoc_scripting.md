---
title: Sl-sh Standard Library
tags: [documentation]
keywords: forms, sl-sh, files, api, standard library
last_updated: March 6, 2021
sidebar: mydoc_sidebar
permalink: mydoc_scripting.html
---

# writing scripts

mkli

## Scripting
Sl-sh supports scripts. If you put a `#!/path/to/sl-sh` at the top
of a file and run `chmod +x` on it, it will be executable. If you pass
arguments to a sl-sh script, the parameters will be available as a vector,
`args`.


### Error reporting
Errors will produce a stack trace as well as a message.



# macros and backquotes
