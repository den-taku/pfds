module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = do
    let a = [1, 2, 3]
    let b = [4, 5, 6]
    let c = append a b
    print $ "a: " ++ show a
    print $ "b: " ++ show b
    print $ "c: " ++ show c
    let d = update c 3 10
    print $ "c: " ++ show c
    print $ "d: " ++ show d
    print $ "member 3 d: " ++ show  (member 3 d)

append :: [a] -> [a] -> [a]
append xs ys = foldr (:) ys xs

update :: [a] -> Int -> a -> [a]
update (x:xs) 0 y = y : xs
update (x:xs) i y = x : update xs (i-1) y

class Set a where
    empty :: a b

    insert :: b -> a b -> a b
    member :: Eq b => b -> a b -> Bool

instance Set [] where
    empty = []

    insert v xs = v : xs
    member v (x:xs) = v == x || member v xs

data (Ord a, Eq a)  => Tree a = Nil | Node (Tree a) a (Tree a) deriving Show

instance Set Tree where
    empty = Nil

    insert x Nil = Node Nil x Nil
    insert x (Node l e r)
        | x == e = Node l e r
        | x < e = Node (insert x l) e r
        | otherwise = Node l e $ insert x r
    member x tree = case tree of
        Nil          -> False
        (Node l e r) -> x == e || if x < e then member x l else member x r
