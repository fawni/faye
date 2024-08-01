;; faye, a pretty lil lisp
;; Copyright (c) 2023 fawn
;;
;; SPDX-License-Identifier: Apache-2.0

(+ 5 33 12.2
    (/ 7 2)
    (* 5.5 2x 3.3) ; this will error because 2x is not a valid numeric literal
    (- 120 53))