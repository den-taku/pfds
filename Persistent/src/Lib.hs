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
    empty :: a b -> Bool
    insert :: b -> a b -> a b
    member :: Eq b => b -> a b -> Bool

instance Set [] where
    empty = null
    insert v xs = v : xs
    member v (x:xs) = v == x || member v xs
