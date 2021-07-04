#!/usr/bin/env sl-sh

(defn myfn nil)
(let*
  ((mything 7))
  (set! myfn (fn (x) (do
                        (set! mything (+ x mything))
                        mything))))

(println (myfn 7))
(println (myfn 14))
(println (myfn 5))

;; (let* (() () ()) ())

(defmacro closure (fn-sym clauses &rest params)
  `(do
    (defn ,fn-sym nil)
    (let* ,clauses
      (set! ,fn-sym (set! ,@params)))))

(println (expand-macro '(closure 'afn ((mything 7)) (+ x mything))))


