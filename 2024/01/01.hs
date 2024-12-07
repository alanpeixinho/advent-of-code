import System.IO
import Data.List
import Data.Typeable
import Data.Map (Map)
import qualified Data.Map as Map
import Data.Maybe
import Text.Printf

read_input :: String -> (Integer, Integer)
read_input s = totuple $ map read $ words s
    where totuple [x, y] = (x, y)
          totuple _ = error "list should have 2 ids"

distance :: (Integer, Integer) -> Integer
distance (x1, x2) = abs (x1 - x2)

countOccurrences :: [Integer] -> Map.Map Integer Integer
countOccurrences xs = foldr (\x acc -> Map.insertWith (+) x 1 acc) Map.empty xs

similarity :: Integer -> Map Integer Integer -> Integer
similarity x h2 = x  * fromMaybe 0 (Map.lookup x h2)

main = do
    all_ids <- fmap (map read_input . lines) $ hGetContents stdin

    let (l1, l2) = unzip all_ids
    printf "total distance: %d\n"  (sum $ map distance $ zip (sort l1) (sort l2))

    let hist2 = countOccurrences l2
    printf "similarity score: %d\n" (sum $ map (\x -> similarity x $ hist2) l1)
