(ns-push 'docstruct)
(ns-import 'shell)
(ns-import 'iterator)

(defn -lst-to-doc-section (accum)
  (apply str (collect-vec (filter (fn (x) (not (str-empty? (str-trim x)))) accum))))

(defn -if-exists (to-find arr)
      (block found-str
         (dotimes-i idx (length arr) (when (str-contains to-find (vec-nth arr idx))
                                       (return-from found-str (values idx (vec-nth arr idx)))))
         (values 0 nil)))

(defn -until
"Given a list of strings, lst, a list of strings, stop-conditions, and a list to append to,
accum, iterate over list of strings, append strings that are non empty and
do not start with one of the strings stop-conditions to accum until line ends or
a line starts with a stop condition. Return values obj.
values-nth 0 = with idx of first string match in stop condition, or 0.
values-nth 1 = accum lst to string.
"
    (lst stop-conditions accum)
    (let ((idx 0))
      (block went-until (do
          (for line in lst
                (let* ((filter-fn (fn (x)
                                     (let ((trim-line (str-trim line)))
                                            (str-starts-with x trim-line))))
                       (should-stop ;; if any of the stop conditions are not filtered
                                    ;; it means a match was found and
                         (not (nil? (collect (filter filter-fn stop-conditions))))))
                  (set! idx (+ idx 1))
                  (if should-stop
                    (do
                      (return-from went-until (values (- idx 1) (-lst-to-doc-section accum))))
                    (do
                      (append-to! accum (list (str (str-trim line) "\n")))))))
              (values 0 (-lst-to-doc-section accum))))))

(defn make-doc-struct (a-sym) (let* ((ret (get-error (let ()
    (def doc-str nil)
    (def sl-sh-type (values nil nil))
    (def namespace (values nil nil))
    (def usage (values nil nil))
    (def desc (values nil nil))
    (def sect (values nil nil))
    (def sect-key "Undefined")
    (def example (values nil nil))
    (def raw-doc (get-error (doc-raw a-sym)))
    (if (or (and (= :ok (car raw-doc))
                 (str-empty? (cdr raw-doc)))
            (not (= :ok (car raw-doc))))
      (set! doc-str (str a-sym "\n\nsee source."))
      (do
        (set! doc-str (doc a-sym))
        (def doc-vec (collect-vec (map str-trim (str-split "\n" doc-str))))
        (when (>= (length doc-vec) 2)
          (set! sl-sh-type (-if-exists "Type:" (vec-slice doc-vec 1 (length doc-vec)))))
        (when (>= (length doc-vec) 2)
          (set! namespace (-if-exists "Namespace" (vec-slice doc-vec 1 (length doc-vec)))))
        (when (>= (length doc-vec) 2) (do
          (set! usage (-if-exists "Usage:" (vec-slice doc-vec 1 (length doc-vec))))
        (when (>= (length doc-vec) 3) (do
          (set! desc (-until (vec-slice doc-vec (+ 2 usage) (length doc-vec))
                            (list "Section:" "Example:")
                            (list)))
        (when (and (> (values-nth 0 desc) 0) ;; if idx is 0, no stop-condition
                   ;; strings were found.
                   (> (length doc-vec) (+ 2 usage desc))) (do
          (set! sect (-until (vec-slice doc-vec (+ 2 usage desc) (length doc-vec))
                              (list "Example:")
                              (list)))
          (when (not (nil? (values-nth 1 sect)))
            (let ((ret (get-error (str-trim (vec-nth (str-split "Section:" (values-nth 1 sect)) 1)))))
              (if (= :ok (car ret))
                (set! sect-key (cdr ret))
                (set! sect-key "Undefined"))))
          (when (and (> (values-nth 0 sect) 0) ;; if idx is 0, no stop-condition
                     ;; strings were found.
                     (> (length doc-vec) (+ 2 usage desc sect))) (do
            (set! example (-until (vec-slice doc-vec (+ 2 usage desc sect) (length doc-vec))
                              (list)
                              (list)))))))))))))
    (struct::defstruct doc-struct
        (full-doc "Full docstring" doc-str :ro)
        (name "Symbol" (str a-sym) :ro)
        (sl-sh-type "Type" (values-nth 1 sl-sh-type) :ro)
        (namespace "Namespace" (values-nth 1 namespace) :ro)
        (usage "Usage" (values-nth 1 usage) :ro)
        (description "" (values-nth 1 desc) :ro)
        (section "Section" (values-nth 1 sect) :ro)
        (section-key "Section key" sect-key :ro)
        (example "Example" (values-nth 1 example) :ro)
        (:fn namespace-only (self)
                    (let* ((ns (self :namespace))
                           (ns-str (str-split ":" ns)))
                      (if (> (length ns-str) 1)
                        (str-trim (vec-nth ns-str 1))
                        "")))
        (:fn to-str (self) (do
            (println (self :full-doc)))))
       (def ds (doc-struct))
       ds))))
        (if (= :ok (car ret))
          (cdr ret)
          (print-error ret))))

(ns-auto-export 'docstruct)
(ns-pop)
