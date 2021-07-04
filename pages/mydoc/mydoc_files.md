---
title: Sl-sh Standard Library
tags: [documentation]
keywords: forms, sl-sh, files, api, standard library
last_updated: March 6, 2021
sidebar: mydoc_sidebar
permalink: mydoc_files.html
---

# sl-sh file api


## reading files

## writing to files

## reading and writing files

## something cool

## redirection, note endfix
# investigating-file-io-forms


```
(touch "/tmp/slsh-pacman.log")
(loop (output-file input-file) ((open "/tmp/slsh-pacman.log" :write) (open "/var/log/pacman.log" :read)) (write-line output-file (read-line input-file)))
```

vs.

cat /var/log/pacman.log out> /tmp/pacman.log


maybe add in a tr in the native stdin-stdout pacman.log case and something equivalent for the pure sl-sh case
<hr>
[<-- back to the docs]({{ site.url }}{% link pages/mydoc/mydoc_api.md %})
