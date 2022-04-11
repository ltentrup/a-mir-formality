#lang racket

(require redex/reduction-semantics
         "../grammar.rkt"
         "../../logic/env.rkt"
         "../../logic/cosld-solve.rkt"
         "../hook.rkt"
         "../unify.rkt"
         )
(provide env-with-clauses-invariants-and-generics
         EmptyEnv
         ty:prove-top-level-goal/cosld
         ty:test-can-prove
         ty:test-cannot-prove
         )

(define-metafunction formality-ty
  ;; Creates a suitable env for testing the ty-logic layer
  ;; that has the given clauses, invariants, and adt-declarations.
  env-with-clauses-invariants-and-generics : Clauses Invariants ((AdtId Generics) ...) -> Env

  [(env-with-clauses-invariants-and-generics Clauses Invariants ((AdtId Generics) ...))
   (env-with-hook (Hook: ,(begin
                            (formality-ty-hook (lambda (predicate) (term Clauses))
                                               (term Invariants)
                                               (lambda (env var-ids predicate1 predicate2)
                                                 (term (ty:equate-predicates/vars ,env ,var-ids ,predicate1 ,predicate2)))
                                               (lambda (env relation)
                                                 (term (ty:relate-parameters ,env ,relation)))
                                               (lambda (predicate1 predicate2) #t)
                                               (lambda (adt-id)
                                                 (term (find-adt-generics ,adt-id ((AdtId Generics) ...))))
                                               ))))
   ]
  )

;; A suitable env for testing the formality-ty layer in isolation
;; with no clauses, invariants, nor generics.
(define-term EmptyEnv (env-with-clauses-invariants-and-generics () () ()))

(define-metafunction formality-ty
  find-adt-generics : AdtId ((AdtId_1 Generics) ...) -> Generics

  [(find-adt-generics AdtId (_ ... (AdtId Generics) _ ...)) Generics]
  )

(define-extended-judgment-form formality-ty logic:prove-top-level-goal/cosld
  ;; See logic:prove-top-level-goal/cosld
  #:mode (ty:prove-top-level-goal/cosld I I O)
  #:contract (ty:prove-top-level-goal/cosld Env Goal Env)

  )

(define-syntax-rule (ty:test-can-prove env goal)
  (test-equal
   (judgment-holds (ty:prove-top-level-goal/cosld env goal _))
   #t)
  )

(define-syntax-rule (ty:test-cannot-prove env goal)
  (test-equal
   (judgment-holds (ty:prove-top-level-goal/cosld env goal _))
   #f)
  )