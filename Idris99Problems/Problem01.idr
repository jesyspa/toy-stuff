module Problem01
-- Find the last element of a list.

myLastV : Vect (S n) a -> a
myLastV [x] = x
myLastV (_::x::xs) = myLastV (x::xs)

myIsCons : List a -> Bool
myIsCons [] = False
myIsCons (x::xs) = True

myLastL : (l : List a) -> (ok : isCons l = True) -> a
myLastL [x] _ = x
myLastL (y::x::xs) _ = myLastL (x::xs) Refl


