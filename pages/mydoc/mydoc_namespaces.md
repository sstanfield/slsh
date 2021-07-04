---
title: Namespaces
tags: [documentation]
last_updated: March 12, 2021
sidebar: mydoc_sidebar
keywords: namespaces
permalink: mydoc_namespaces.html
---
# Using namespaces in sl-sh
<hr>

## ns push and pop
The [namespace functions](/pages/mydoc/mydoc_api.html#Namespace%20forms-body) are
used to create and modify namespaces in sl-sh. `ns-push` 
creates and/or enters a namespace, and `ns-pop` exits the current namespace.

### ns-push
{% comment %} hiding this from md output
    {% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "ns-push-docs.lisp")))) {% endcomment %}
```
    #!/usr/bin/env sl-sh
    (println (doc 'ns-push))
    ```
{% endcomment %}
```
$> (doc 'ns-push)
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "ns-push-docs.lisp"))))) {% endcomment %}
;; ns-push
;; Type: Macro
;; Namespace: root
;; 
;; Usage: (ns-push 'namespace)
;; 
;; Pushes the current namespace on a stack for ns-pop and enters or creates namespace.
;; 
;; Section: namespace
;; 
;; Example:
;; (def test-ns-push *ns*)
;; (ns-push 'ns-pop-test-namespace)
;; ; *ns* will not be right...
;; (test::assert-equal "ns-pop-test-namespace" *active-ns*)
;; (ns-push 'ns-pop-test-namespace2)
;; (test::assert-equal "ns-pop-test-namespace2" *active-ns*)
;; (ns-pop)
;; (test::assert-equal "ns-pop-test-namespace" *active-ns*)
;; (ns-pop)
;; (test::assert-equal test-ns-push *ns*)
;; 
;; 
;; 
```

### ns-pop
{% comment %}

    {% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "ns-pop-docs.lisp")))) {% endcomment %}
```
    #!/usr/bin/env sl-sh
    (println (doc 'ns-pop))
    ```
{% endcomment %}

```
$> (doc 'ns-pop)
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "ns-pop-docs.lisp"))))) {% endcomment %}
;; ns-pop
;; Type: Macro
;; Namespace: root
;; 
;; Usage: (ns-pop)
;; 
;; Returns to the previous namespace saved in the last ns-push.
;; 
;; Section: namespace
;; 
;; Example:
;; (def test-ns-pop *ns*)
;; (ns-push 'ns-pop-test-namespace)
;; (test::assert-equal "ns-pop-test-namespace" *active-ns*)
;; (ns-pop)
;; (test::assert-equal test-ns-pop *ns*)
;; 
;; 
;; 
```

## A basic example

any functions we want namespaced must occur between calls to ns-push and ns-pop.

`./basic-1-add.lisp`
{% comment %} (def directive (make-hash (list (join :type :lib) (join :name "basic-1-add.lisp")))) {% endcomment %}
```
(ns-push 'add)

(defn plus1 (x)
    (+ x 1))

(ns-pop)
```

here, the function, `plus1` is now associated with the namespace `add`. Let's try and use it.

`basic-ns-test.lisp`
{% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "basic-ns-test.lisp")))) {% endcomment %}
```
#!/usr/bin/env sl-sh

(load "./basic-1-add.lisp")

(println (add::plus1 8))
```

result:
```
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "basic-ns-test.lisp" "basic-1-add.lisp"))))) {% endcomment %}
;; 9
```

{% include note.html content="the use of the load function. This allows the contents of the file `basic-1-add.lisp` to be evaluated in the current file." %}
{% include note.html content="the explicit call to `plus1` with `add::plus1`. In sl-sh, namespaced symbols can be accessed with the double colon :: operator. `plus1` is 'namespaced' under the symbol `add`, and available after the call to load." %}


## exporting and importing

### ns-export
{% comment %} hiding this from md output
    {% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "ns-export-docs.lisp")))) {% endcomment %}
```
    #!/usr/bin/env sl-sh
    (println (doc 'ns-export))
    ```
{% endcomment %}
```
$> (doc 'ns-export)
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "ns-export-docs.lisp"))))) {% endcomment %}
;; ns-export
;; Type: Macro
;; Namespace: root
;; 
;; Usage: (ns-export symbol_or_sequence)
;; 
;; Export a symbol or list of symbols to be imported into other namespaces.
;; 
;; Section: namespace
;; 
;; 
```

### ns-auto-export
{% comment %}

    {% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "ns-auto-export-docs.lisp")))) {% endcomment %}
```
    #!/usr/bin/env sl-sh
    (println (doc 'ns-auto-export))
    ```
{% endcomment %}

```
$> (doc 'ns-auto-export)
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "ns-auto-export-docs.lisp"))))) {% endcomment %}
;; ns-auto-export
;; Type: Macro
;; Namespace: root
;; 
;; Usage: (ns-auto-export symbol)
;; 
;; Macro that takes a symbol, the symbol of the current namespace, and writes an
;; ns-export statement that includes all symbols defined in the namespaces scope
;; that do not begin with the '-' symbol. This is a convenience method that allows
;; user to avoid enumerating all symbols while also introducing a mechanism to
;; exclude symbols from being excluded. Note, if using ns-auto-export, it is
;; not possible to export a symbol that is already defined in another namespace,
;; if said functionality is desired the symbol must be manually exported with
;; another ns-export statement; ns-auto-export can be used in conjunction with
;; ns-export.
;; 
;; Section: namespace
;; 
;; 
```

### ns-import
{% comment %}

    {% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "ns-import-docs.lisp")))) {% endcomment %}
```
    #!/usr/bin/env sl-sh
    (println (doc 'ns-import))
    ```
{% endcomment %}

```
$> (doc 'ns-import)
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "ns-import-docs.lisp"))))) {% endcomment %}
;; ns-import
;; Type: Macro
;; Namespace: root
;; 
;; Usage: (ns-import namespace)
;; 
;; Import any symbols exported from namespace into the current namespace.
;; 
;; Section: namespace
;; 
;; 
```


## An example using exports and imports

Exporting symbols from a given namespace allows those symbols to be imported
directly into other namespaces. This allows referencing def'd symbols without
the aforementioned :: syntax.

`add1.lisp`
{% comment %} (def directive (make-hash (list (join :type :lib) (join :name "add1.lisp")))) {% endcomment %}
```
(ns-push 'add1)

(defn add1 (x)
	(+ x 1))

(ns-export '(add1))
```


`add2.lisp`
{% comment %} (def directive (make-hash (list (join :type :lib) (join :name "add2.lisp")))) {% endcomment %}
```
(load "./add1.lisp")
(ns-push 'add2)
(ns-import 'add1) ;; ns-import call is after call to load.

(defn -make-adder (to-add iterations)
	(loop (add iter) (to-add 0) (if (> iter (- iterations 1)) add (recur (add1 add) (add1 iter)))))

(defn add2 (x)
	(-make-adder x 2))

(ns-auto-export 'add2)
(ns-pop)
```

`ns-test.lisp`
{% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "ns-test.lisp")))) {% endcomment %}
```
#!/usr/bin/env sl-sh

(load "./add2.lisp")
(ns-push 'ns-test)
(ns-import 'add2)

(println (add2 8))

;; -make-adder can only be accessed through the add2 namespace, it was not
;; exported with ns-auto-export because it is prefixed with '-'.
(println ((fn (x) (add2::-make-adder x 5)) 8))

(ns-pop)
```


result:
```
./ns-test.lisp
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "ns-test.lisp" "add1.lisp" "add2.lisp"))))) {% endcomment %}
;; 10
;; 13
```

## Helpers
### the load path
The load path refers to the sl-sh global variable `*load-path*`. This variable
controls where lisp files are loaded from. By default, calls to load work for
absolute and relative links, this is why in the previous examples, lines like:
```
(load "./add1.lisp")
```
work fine. The entrypoint script is given a path directly to a file. Lisp
files on the load path can be accessed by name, without the need for directory
syntax like:
```
(load "my-helper-script.lisp")
```
sl-sh looks for the file named `my-helper-script.lisp` in each directory on
the load path. By default the load path is equivalent to the list
`("~/.config/sl-sh/"). To add a new directory to the load-path run:
```
(set! *load-path* (iterator::append-to! *load-path* '("/path/to/new/directory/")))'))
```

The load path can also be overriden entirely with a custom set of directories:
```
(set! *load-path* '("/path/to/dir1/" "/path/to/dir2/"))
```

{% include note.html content="Global variables def'd in sl-sh by convention are surrouned by the asterisk symbol, *, on both sides, these are called earmuffs." %}

### mkli - MaKe LIsp
a function that creates an executable sl-sh script for you.

{% comment %} hiding this from md output
    {% comment %} (def directive (make-hash (list (join :type :entrypoint) (join :name "mkli.lisp")))) {% endcomment %}
```
    #!/usr/bin/env sl-sh

    (println (doc 'mkli))
    ```
    result:

{% endcomment %}
```
./mkli.lisp
{% comment %} (def directive (make-hash (list (join :type :eval) (join :files (list "mkli.lisp"))))) {% endcomment %}
;; mkli
;; Type: Lambda
;; Namespace: shell
;; 
;; Usage: (mkli filepath [namespace] [body])
;;     "make lisp".creates a sl-sh shell script. given a file, a namespace (optional 2nd arg), and a string
;;     to populate as the body (optional 3rd arg), make a canonincal blank sl-sh script
;;     complete with all the relevant imports, and boilerplate namespace code taken
;;     care of to speed up development.
;; 
;;     It is recommended all calls to load are done at the top of the file (before
;;     the calls to ns-enter or ns-create, in case a library sl-sh script calls a
;;     library sl-sh script that created a namespace and forgot to call ns-pop.
;;     This ensures the exported symbols for the first library's scripts
;;     namespace are importable in the executing script's namespace.
;; 
;;     All calls to ns-import happen after a ns is created and entered so the
;;     current namespace is the namespace that houses all the imported symbols.
;; 
;;     ns-export must be called before ns-pop so the appropriate symbols are
;;     associated namespace, the one in which the symbols were created.
;; 
;;     Section: scripting 
;; 
;; 
```
use mkli immediately once something in the repl starts to become unwieldy.
For instance, if the following command is executed:
```
(for file in (map str-trim (str-split :whitespace (str (find $PWD -iname "*.log")))) (do (println "File: " file)))
```
All log files in current and child directories will be printed. If the desired
goal is to do some custom processing on each file. Next execute:
```
mkli find-logs.lisp logfinder *last-command*
```

`*last-command*` is a global sl-sh variable (note the earmuffs!) that is always
set to the last run command. Passing it along with the two other parameters
to `mkli` produces:
`find-logs.lisp`
```
#!/usr/bin/env sl-sh

(ns-push 'logfinder)
(ns-import 'shell)

(for file in (map str-trim (str-split :whitespace (str (find $PWD -iname "*.log")))) (do (println "File: " file)))

(ns-auto-export 'logfinder)
(ns-pop)
```
This pattern allows for easily transitioning from workshopping commands on the
shell to creating executable shell scripts when more complex syntax is necessitated.



[<-- back to the docs]( {{ site.url }} )

