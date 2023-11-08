;; faye, a pretty lil lisp
;; Copyright (c) 2023 fawn
;;
;; SPDX-License-Identifier: Apache-2.0

(fn factorial [n]
  (if (< n 2)
    1
    (* n (factorial (- n 1)))))

(println (factorial 5))