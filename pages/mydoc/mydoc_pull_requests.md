---
title: Pull Requests
keywords: contributing, github, pull requests
tags: [contributing]
summary: "Guide for contributors"
sidebar: mydoc_sidebar
permalink: mydoc_pull_requests.html
---

sl-sh welcomes pull requests via GitHub: fixing bugs, adding features,
enhancing builtins, adding to the standard library, or anything else.

## Contributing Rust code

*  all submitted rust code must pass the rust fmt and clippy checks, it is
built in to the CI. see `.github/workflows/rust.yml` for reference.
```
cargo fmt
cargo clippy
```

## Contributing sl-sh code

*  all sl-sh code must be documented. The CI automatically runs sl-sh tests.
Therefore, all new sl-sh functions must include a docstring that has an 
`Example: ` section with a sufficient number of test cases (the test library
is made available automatically in this section) to verify correctness.

## Checking out issues

*  peruse [open issues](https://github.com/sl-sh-dev/sl-sh/issues?q=is%3Aopen+is%3Aissue)
if you want to get started right away.
