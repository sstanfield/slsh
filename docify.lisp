#!/usr/bin/env sl-sh

;; TODO general
;; - could put all functions in glossary type datastructure, would make them searchable potentially?
;;      would be cool if search feature of website worked for docs too.
;; - get /news panel back!
;; - fix syntax highlighting in html output.


(load "mk-docs.lisp")
(load "mk-post.lisp")
(load "mk-sidebar.lisp")

(ns-import 'iterator)
(ns-import 'shell)
(ns-import 'mkdocs)
(ns-import 'mkpost)
(ns-import 'mksidebar)

(error-stack-on)

(defn list-of-all-slsh-syms ()
    (let ((sym-list (qsort (collect (filter-undocable-forms (hash-keys *std-lib-exported-syms-hash*))))))
    sym-list))

(let ((result (get-error (let ((sym-list (list-of-all-slsh-syms)))
  ;; TODO last updated line frontmatter line should update automatically.
  ;; create std lib md file
  (write-sidebar "_data/sidebars/mydoc_sidebar.yml"  sym-list "Standard Library" "Sl-sh Forms" "/mydoc_api.html")
  (make-md-file "pages/mydoc/mydoc_api.md" sym-list)
  (eval-post "_evalable_pages/mydoc/mydoc_documentation.md" "pages/mydoc/mydoc_documentation.md")
  (eval-post "_evalable_pages/mydoc/mydoc_shellreader.md" "pages/mydoc/mydoc_shellreader.md")
  (eval-post "_evalable_pages/mydoc/mydoc_reader.md" "pages/mydoc/mydoc_reader.md")
  (eval-post "_evalable_pages/mydoc/mydoc_namespaces.md" "pages/mydoc/mydoc_namespaces.md")))))
  (if (= (car result) :ok)
    (cdr result)
    (print-error result)))
