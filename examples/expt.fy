(fn expt
  [x n]
  (if (= n 0)
    1
    (* x (expt x (- n 1)))))


(expt 2 16)
