#!/usr/bin/env sl-sh

(load "docstrings-to-md.lisp")

(ns-push 'mkdocs)
(ns-import 'iterator)
(ns-import 'shell)
(ns-import 'struct)
(ns-import 'test)
(ns-import 'docmd)

(defn filter-undocable-forms (sym-list)
	(filter (fn (x)
		(and
			(not (= x 'filter-undocable-forms))
			(not (= x '*ns*))
			(not (= x '*euid*))
			(not (= x '*uid*))
			(not (= x '*ns-exports*))
			(not (= x '^ns-stack-xyz^))
			(not (= x 'tok-slsh-form-color))
			(not (= x 'tok-slsh-fcn-color))
			(not (= x 'tok-default-color))
			(not (= x 'tok-sys-command-color))
			(not (= x 'tok-sys-alias-color))
			(not (= x 'tok-string-color))
			(not (= x 'tok-invalid-color))
			(not (= x '*run-script*))
			(not (= x '*active-ns*))
			(not (= x 'internal-fn))
			(not (= x 'internal-macro))
			(not (= x 'make-md-file))
			(not (= x '__completion_hook))
			(not (= x '__line_handler))
			(not (= x '__exec_hook))
			(not (= x '__prompt))
			(not (= x 'list-of-all-slsh-syms))
			(not (= x 'args))))
		sym-list))

(defn make-md-file
"Generate slsh standard library documentation md page."
(index-file sym-list)
     ;;TODO this or requires too much esoteric knowledge to be a good callee
     ;; need a common way to ask if something is useable on the std lib's 
     ;; seq/iter stuffz.
     ;;TODO need a few macros around get-error to make it less verbose to deal with,
     ;; maybe form, if success, if failure
      (if (or (seq? sym-list) (iter? sym-list))
          (gen-std-lib-md-file index-file (qsort sym-list))
          (err "second argument ot make-md-file must be keyword :single or :lang
               or a sequence of sl-sh symbols")))

(ns-auto-export 'mkdocs)

(ns-pop)
