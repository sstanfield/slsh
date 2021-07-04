---
title: "Getting started with sl-sh"
keywords: sample homepage
tags: [getting_started]
sidebar: mydoc_sidebar
permalink: index.html
summary: These brief instructions will help you get started with sl-sh.
---


## Install sl-sh

Follow these instructions to install sl-sh.

### 1. Install Rust
-  Documentation [here](https://www.rust-lang.org/tools/install)

### 2. Build sl-sh
-  Get source and build
    ```
    git clone https://github.com/sl-sh-dev/sl-sh
    cd slsh
    cargo build --release
    ./target/release/sl-sh
    ```
-  sl-sh will load with the default slshrc file.

### 3. Use sl-sh as primary shell
-  install binary
```
sudo install -D -m 755 target/release/sl-sh /usr/local/bin/
```

-  add sl-sh to /etc/shells and change login shell to sl-sh
```
echo /usr/local/bin/sl-sh | sudo tee -a /etc/shells
chsh -s /usr/local/bin/sl-sh
```

{% include links.html %}

This site built from [Documentation Theme for Jekyll](https://github.com/tomjoht/documentation-theme-jekyll).
