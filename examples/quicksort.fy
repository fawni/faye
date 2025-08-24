(fn cons [x y] (list x y))
(fn car [cell] (nth cell 0))
(fn cdr [cell] (nth cell 1))
;; church encoding. about 4x slower. good for catching bugs!
;(fn cons [x y] (lambda [f] (f x y)))
;(fn car [cell] (cell (lambda [x y] x)))
;(fn cdr [cell] (cell (lambda [x y] y)))

(fn consify' [v i]
  (if (= i (len v))
    nil
    (cons (nth v i) (consify' v (+ i 1)))))
(fn consify [v] (consify' v 0))

(fn strlist [cell]
  (if (= nil cell)
    "nil"
    (let (ar (car cell)) (dr (cdr cell))
      (if (= nil dr)
        (str ar)
        (str ar ", " (strlist dr))))))

(fn filter [f source]
  (if (= nil source)
    nil
    (let (ar (car source)) (dr (cdr source))
      (if (and (= nil ar) (= nil dr))
        source
        (if (f ar)
          (cons ar (filter f dr))
          (filter f dr))))))

(fn concat [a b]
  (if (or (= nil a) (= nil b))
    (if (= nil a)
      b
      a)
    (let (ar (car a)) (dr (cdr a))
      (if (= nil dr)
        (cons ar b)
        (cons ar (concat dr b))))))

(fn quicksort [unsorted]
  (if (= nil unsorted)
    nil
    (let (head (car unsorted)) (tail (cdr unsorted))
      (if (= nil tail)
        unsorted ; a list with 0 or 1 items is already sorted
        (concat
          (quicksort (filter (lambda [x] (<= x head)) tail))
          (cons head
            (quicksort (filter (lambda [x] (> x head)) tail))))))))

(const unsorted (consify [5 1 2 6 3 9 7 10 8 4]))
(println "unsorted:" (strlist unsorted))
(println "  sorted:" (strlist (quicksort unsorted)))
