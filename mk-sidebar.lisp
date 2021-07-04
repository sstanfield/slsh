#!/usr/bin/env sl-sh

(load "docstruct.lisp")
(load "mk-docs.lisp")

(ns-push 'mksidebar)

(ns-import 'shell)
(ns-import 'iterator)
(ns-import 'docstruct)
(ns-import 'mkdocs)

(defn make-section (sym-list title sub-title url)
        (let* ((sections-str (str-push! (str)
                #\u{a} #\u{a}
                #\u{20} #\u{20}
                "- title:" #\u{20} title
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20}
                "output: web, pdf"
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20}
                "folderitems:"
                #\u{a} #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20}
                "- title: " sub-title
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                "url: " url
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                "output: web, pdf"
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                "subfolders:"
                #\u{a} #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                "- title: Quick Links"
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                "output: web"
                #\u{a}
                #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                "subfolderitems:"
                #\u{a} #\u{a}))
            (section-xform (fn (section-name)
                (str-push! (str)
                           #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                           "- title:" #\u{20} section-name
                           #\u{a}
                            #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                           "  url: /mydoc_api.html#" section-name "-body"
                           #\u{a}
                           #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20} #\u{20}
                           "  output: web"
                           #\u{a} #\u{a}
                           )))
            )
          (for section in (qsort (hash-keys (reduce (fn (fst nxt)
            (do (hash-set! fst (nxt :section-key) nil))) (make-hash) (map make-doc-struct sym-list))))
               (str-push! sections-str (section-xform section)))
              sections-str))

(defn -get-sidebar-yml (sym-list title sub-title url)
      (let ((yml (str "
# This is your sidebar TOC. The sidebar code loops through sections here and provides the appropriate formatting.

entries:
- title: sidebar
  product: sl-sh doc
  version: $((vec-nth (str-split :whitespace (version)) 1))
  folders:

  - title:
    output: pdf
    type: frontmatter
    folderitems:
    - title:
      url: /titlepage.html
      output: pdf
      type: frontmatter
    - title:
      url: /tocpage.html
      output: pdf
      type: frontmatter

  - title: Overview
    output: web, pdf
    folderitems:

    - title: Get started
      url: /index.html
      output: web, pdf
      type: homepage

    - title: Introduction
      url: /mydoc_introduction.html
      output: web, pdf

    - title: Support
      url: /mydoc_support.html
      output: web, pdf

    - title: FAQ
      url: /mydoc_faq_layout.html
      output: web, pdf

  - title: Lisp
    output: web, pdf
    folderitems:

    - title: Documentation
      url: /mydoc_documentation.html
      output: web, pdf

    - title: Reader
      url: /mydoc_reader.html
      output: web, pdf

"
(make-section sym-list title sub-title url)

"
  - title: Shell
    output: web, pdf
    folderitems:

    - title: Shell Reader
      url: /mydoc_shellreader.html
      output: web, pdf

    - title: Namespaces
      url: /mydoc_namespaces.html
      output: web, pdf

    - title: Slshrc config
      url: /mydoc_slshrc_config.html
      output: web, pdf

    - title: Working with files
      url: /mydoc_files.html
      output: web, pdf

    - title: Endfix notation on the REPL
      url: /mydoc_endfix.html
      output: web, pdf

    - title: Scripting
      url: /mydoc_scripting.html
      output: web, pdf

    - title: Pipes
      url: /mydoc_pipes.html
      output: web, pdf

  - title: Release Notes
    output: web, pdf
    folderitems:

    - title: 1.0 Blockers
      url: /mydoc_blockers_1_0.html
      output: web, pdf

  - title: Contributing
    output: web, pdf

    folderitems:
    - title: Pull Requests
      url: /mydoc_pull_requests.html
      output: web, pdf")))
      yml))

(defn write-sidebar (dest-filename sym-list title sub-title url)
    (let ((dest-file (open dest-filename :create :truncate)))
      (for line in (str-split "\n" (-get-sidebar-yml sym-list title sub-title url))
        (write-line dest-file line))
      (close dest-file)))

(ns-auto-export 'mksidebar)
(ns-pop)
