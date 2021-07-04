---
title:  "Documentation in Sl-sh"
tags: [documentation]
last_updated: May 28, 2021
sidebar: mydoc_sidebar
keywords: documentation
permalink: mydoc_documentation.html
---

# Sl-sh documentation


The [sl-sh documentation]({{ site.url }}) relies on github pages which is in turn powered by jekyll to build
documentation. In the docs/ sub directory of the sl-sh root project all
of the resources needed to run the docs locally can be found.

## Background
Sl-sh relies on the [Jekyll Documentation Theme](https://idratherbewriting.com/documentation-theme-jekyll/index.html) and utilizes a sl-sh script, `./docify.lisp` to programmatic-ally edit key
resources and insert all of the documentation/pages/post needed to generate
the sl-sh static documentation site.

Read through the README.docker to get a docker container running that will
automatically watch the docs directory and re-build the site on localhost:4000
if any files change.


Running the `./docify` script builds the version of the sl-sh documentation site
for the standard library, however, it is possible to generate a version of the
site that builds documentation for all user created forms.

## Writing sl-sh docs
Sl-sh optionally allows the third argument to def/defn to be a doc string.
Here is the doc string for the symbol `'for'`
```
"
Loops over each element in an iterator.  Will call iter on the input object.
bind is bound to the current element of items and is accesible
in body. body is evaluated a number of times equal to the the number of items
in in_list.

Section: iterator

Example:
(def i 0)
(iterator::for x in (iterator::range 11) (set! i (+ 1 i)))
(assert-equal 11 i)
"
```
Doc strings have the following structure:
```
[sl-sh] {[ frostig ]}: ~/dvlpmnt/slsh/ (docs/update)
λ⸨⚙ ⸩ → doc for
for
Type: Macro
Namespace: iterator

Usage: (for bind in items body)


Loops over each element in an iterator.  Will call iter on the input object.
bind is bound to the current element of items and is accesible
in body. body is evaluated a number of times equal to the the number of items
in in_list.

Section: iterator

Example:
(def i 0)
(iterator::for x in (iterator::range 11) (set! i (+ 1 i)))
(assert-equal 11 i)
```
First and foremost ORDER MATTERS and so do COLONS. All these sections must occur in exactly
the order listed above and they must be on their own line and they must be
immediately followed by a colon, no exceptions (unless you like errors).

### Symbol name, Type, Namespace, Usage
The Symbol name, Type, Namespace, and Usage sections are automatically
generated. The first line printed is the symbol name. Type
refers to the type of the symbol. Namespace refers to the namespace in which
the symbol is declared. Usage shows how the symbol is used.

Usage can be manually provided in the doc string if needed.

### Description, Section, and Example
The paragraph/section after Usage and before Section is called the description.
The key here is to be descriptive about what the symbol is/does/is used for/etc.

The Section tag can be anything. Make sure to pick string values that remain
consistent to create logical groupings for the sake of documentation
presentation. Each section used in a doc will be heading on the user docs page.

Optionally (but encouraged) is the Example section. Here tests that
illustrate the use / verify correctness of the symbol can be written.
For examples of how to use/import the sl-sh test api check out the lisp
files in the lisp/ sub directory. Almost (soon to be all!) all of the sl-sh
functions in the standard library include documentation.

## Building personal documentation in slshrc

The ~/.config/sl-sh/slshrc file runs when sl-sh starts up. All forms created
in this file are automatically added to the `'user` namespace. It is possible
and encouraged to load and import other namespaces into your slshrc to
create re-usable code, see the [namespaces](/mydoc_namespaces.html) article
for more context.

Add the following code to your slshrc to automatically generate sl-sh docs
for all user declared symbols.

Notes:

 - make sure to edit the call to pushd "/path/to/sl-sh/docs" to match
the actual path on your machine.
 - the current default is to use the find command and only regenerate the
docs if the api page has not changed in a day. Since the slshrc is loaded
every time a new shell is loaded this is desired so the docs aren't always
regenerated. If needed, just delete the "pages/mydoc/mydoc_api.md" page
and then start a new shell to regenerate the docs.
 - to actually see the docs site you must run the jekyll docker container. To
have personal docs for your personal shell functions forever run the command
in the README.docker that lets the docker daemon handle starting the jekyll
container on reboot to persist personal docs site.

```
(def api-page-name "mydoc_api.md")
(def sidebar-page-name "mydoc_sidebar.yml")
(def api-page (str "pages/mydoc/" api-page-name))
(def sidebar-page (str "_data/sidebars/" sidebar-page-name))
(shell::pushd "/path/to/sl-sh/docs")
(def docs-error-ret
    (get-error (when (or
        (nil? (fs-exists? api-page))
        (= api-page (str-trim (str $(find $api-page -type f -mmin "+1440"))))) (do

    (load "mk-docs.lisp")
    (load "mk-sidebar.lisp")

    (let* ((ns-to-filter-out (append-to! (list "mksidebar" "mkdocs") *std-lib-namespaces*))
          (almost-proper-sym-list (iterator::reduce
                (fn (fst nxt)
                    (append-to! fst (ns-symbols nxt)))
                (list)
                (filter (fn (x) (not (in? ns-to-filter-out x))) (ns-list))))
          (proper-user-sym-list-no-std-lib-defs (collect (iterator::filter
                (fn (x)
                    (not (in? (hash-keys *std-lib-syms-hash*) x)))
                almost-proper-sym-list))))
        (mkdocs::make-md-file
            api-page
            proper-user-sym-list-no-std-lib-defs)
        (mksidebar::write-sidebar sidebar-page proper-user-sym-list-no-std-lib-defs "User Standard Lib" "User Forms" "/mydoc_api.html"))

    (println "Built docs!")))))
(shell::popd)
(if (not (= (car docs-error-ret) :ok))
    (println "Failed to build docs: " (cdr docs-error-ret)))
```

## Viewing personal docs

After re-loading slshrc file and triggering the above code to run (either by
deleting pages/mydoc/mydoc_api.md or by loading slshrc when said file has not
been modified in one day) when you visit localhost:4000 the sidebar will say
"User Standard Lib" instead of "Standard Library" clicking on that link will
reveal all the forms you created in your slshrc and any exported symbols you
import into your slshrc. The Quick Links section will show a list of all
the sections you defined in the user doc strings for the `Section:` tag and
all user docs will be grouped under the appropriate section heading.

Hopefully this will make it easier to create LOTS of custom sl-sh functions,
while still maintaining visibility into what you've down historically to make
organization easier.
