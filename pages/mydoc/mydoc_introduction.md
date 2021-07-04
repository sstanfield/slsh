---
title: Introduction
sidebar: mydoc_sidebar
permalink: mydoc_introduction.html
folder: mydoc
---

## Overview

Simple Lisp SHell (sl-sh) is a lisp based shell written in Rust. It is not POSIX
compliant and makes no effort to be. It runs on the Linux and MacOS platforms.
It is a Lisp-1 that is heavily inspired by Clojure and Common Lisp. It is a 
shell, it is a scripting language, and it is a REPL.

Some of the more prominent features:

* The [shell reader](https://sl-sh-dev.github.io/sl-sh/mydoc_shellreader.html) supports endfix notation so familiar bash-isms like
    ```bash
    cat file | tr -s " " | cut -d " " -f 2,4
    ```
    "just work"
* Support for an rc file, [slshrc](https://sl-sh-dev.github.io/sl-sh/mydoc_slshrc_config.html), to set up environment and fully customize your prompt.
* Commpon Lisp style macro system with support for quote and backquote (with , and ,@ expansion).
* Rich set of types and cohesive standard library built around them: vectors, lists, iterators, file forms, hash maps, pairs, strings, integers, chars, and booleans.
* OO functionality with lisp style [defstruct](https://sl-sh-dev.github.io/sl-sh/mydoc_api.html#struct::defstruct) and [deftrait](https://sl-sh-dev.github.io/sl-sh/mydoc_api.html#struct::deftrait).
* Dynamically Typed
* Mutable state (it's a shell!) but support for functional idioms is built into the standard lib, e.g. filter, reduce, apply, map, etc.
* Import system with [namespaces](https://sl-sh-dev.github.io/sl-sh/mydoc_namespaces.html) to make writing modular scripts and/or library code easier.
* Common Lisp style keyword symbols with colon, `:a-keyword`.
* Clojure style [threading macros](https://clojure.org/guides/threading_macros) and scheme style [pipeline operators](https://srfi.schemers.org/srfi-197/srfi-197.html).

## Getting started

To get started, see [Getting Started][index].

{% include links.html %}
