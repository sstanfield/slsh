(ns-push 'mkpost)
(ns-import 'shell)
(ns-import 'iterator)

(defn -make-jekyll-post-file
	"execute in docs dir, create a jekyll style post. pass in a destination
	directory, title of post, the header, for the post, and optionally,
	categories.
	Section: post"
	(post-dir title header &rest categories)
    (let*
      ((post-body (str "---
layout: default
title: " title "
categories: [" (if (= 0 (length categories)) "general" (str-cat-list "," categories)) "]
---
# " header "
<hr>
[<-- back to the docs]( {{ site.url }} )
"))
	(date (str-trim (str $(date +%Y-%m-%d))))
	(post (open (str post-dir "/evalable-" date "-" (str-replace title " " "-") ".md") :create :append)))
	(write-line post post-body)
	(close post)))

(defmacro make-draft
	"execute in docs dir, create a new draft. pass in title of post, the header,
	for the post, and optionally, categories.
	Section: post"
	(title header &rest categories)
	`(-make-jekyll-post-file "_posts" ,title ,header ,@categories))

(defmacro make-post
	"execute in docs dir, create a new post. pass in title of post, the header,
	for the post, and optionally, categories.
	Section: post"
	(title header &rest categories)
	`(-make-jekyll-post-file "_posts" ,title ,header ,@categories))

(def begin-comment "{% comment %}")
(def end-comment "{% endcomment %}")
(def code-block-delim "```")

(defn -get-directive-hash-map
	"By convention sl-sh hashmaps in lisp strings exit betwen liquid templating
	comment blocks located on the same line. these hash maps are directives that
	aid in transforming an evalable md file into an evaled md file.
	also, continue to write the line in the evalable md file into the
	evaled md file.
	Section: post"
	(src-file dest-file) (block file-read
	(let ((line (read-line src-file)))
	(when (nil? line)
		(return-from file-read nil))
	(do
		(write-string dest-file line)
		(if (and
			(str-contains begin-comment line)
			(str-contains end-comment line))
		(let ((directive-metadata (vec-nth (str-split begin-comment (vec-nth (str-split end-comment line) 0)) 1)))
			(eval (read directive-metadata))
			(hash-set! directive :trigger-line line)
			directive)
		(do
			(recur src-file dest-file)))))))

(defn -read-in-code-block
"read in code block from src-file into the code-snippets map.  also,
continue to write the lines in the evalable md file into the evaled
md file.
Section: post"
	(src-file dest-file directive-map code-snippets)
	(let ((line (str-trim (read-line src-file)))
          (contents (list)))
	(write-line dest-file line)
	(println "codeblockstart::: " line)
	(when (not (= line code-block-delim))
		(err "First line after :entrypoint or :lib directive must be ```"))
	;; read code block into list of strings, store list in directive map as
	;; contents and return.
	(loop (src-file contents) (src-file contents) (do
		(set! line (read-line src-file))
		(write-string dest-file line)
		(set! line (str-trim line))
		(println "code: " line)
		(if (= line code-block-delim)
			contents
			(recur src-file (append-to! contents (list line))))))
	(hash-set! directive-map :contents contents)
	(hash-set! code-snippets (hash-get directive-map :name) directive-map)
	directive-map))

(defn -eval-file (src-file dest-file directive-map code-snippets)
    (let ((temp-dir (str-replace (str $(mktemp -d)) "\n" ""))
            (entrypoint nil))
	;; write all the files to a temp directory so the entrypoint(s) can be
	;; evaled
	(for file-name in (hash-get directive-map :files)
       (let* ((snippet (hash-get code-snippets file-name))
		(target-file-name (str temp-dir "/" file-name))
		(target-file (open target-file-name :create :truncate)))
		(println "target-file-name: " target-file-name)
		(println "contents: " (hash-get snippet :contents))
		(for line in (hash-get snippet :contents)
			(write-line target-file line))
		(when (= :entrypoint (hash-get snippet :type)) (do
			(set! entrypoint target-file-name)
			$(chmod +x $target-file-name)))
		(close target-file)))
	(when (nil? entrypoint) (err "No defined for :type :entrypoint in files found in :files for given :eval directive."))
	(pushd temp-dir)
	;; TODO is the eval needed
	;; eval executable file and write output to temp-out
	(let* ((temp-out (str temp-dir "/output"))
          (return-value (get-error (out-err> temp-out (load entrypoint)))))
	(popd)
	(loop (input-file) ((open temp-out :read))
        (let ((line (read-line input-file)))
			(when (not (nil? line)) (do
                (write-string dest-file (str ";; " line))
				(recur input-file)))))
	(when (not (= (car return-value) :ok))
      (do
        (write-line dest-file (str "Error evaluating entrypoint!"))
        (write-line dest-file (str "==> " (cdr return-value)))))
    (println "codeblock lives: " temp-out))))

(defn -eval-post
"TODO enumerate different directive and expectations
Section: scripting"
    (src-file dest-file code-snippets)
    (let ((directive-map (-get-directive-hash-map src-file dest-file)))
    ;;(println "src-file: " src-file)
    ;;(println "dest-file: " dest-file)
    ;;(println "code-snippets: " code-snippets)
    (when (not (nil? directive-map)) (do
        (match (hash-get directive-map :type)
            ;; :entrypoint and :lib directives indicate that directly
            ;; below is a git flavored markdown (gfm) code block that
            ;; needs to be read into the code-snippets hashmap. The code
            ;; snippets hashmap is where all :entrypoint and :lib code
            ;; is stored.
            (:entrypoint (-read-in-code-block src-file dest-file directive-map code-snippets))
            (:lib (-read-in-code-block src-file dest-file directive-map code-snippets))
            (:eval (-eval-file src-file dest-file directive-map code-snippets))
            (nil (err "Unknown hash map found")))
        (recur src-file dest-file code-snippets)))))

(defn eval-post
	"enumerate different directive and expectations
	Section: scripting"
	(evalable-post-file-name evaled-post-file-name)
    (let ((code-snippets (make-hash))
            (src-file (open evalable-post-file-name :read))
            (dest-file (open evaled-post-file-name :create :truncate)))
		(-eval-post src-file dest-file code-snippets)
		(close src-file)
		(close dest-file)))

(ns-auto-export 'mkpost)
(ns-pop)
