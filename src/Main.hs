module Main where

import Monte
import System.Environment (getArgs)

main :: IO ()
main = do
  -- putStrLn $ fmap show $ fmap Monte.estimatePi 100 $ fmap read $ fmap first getArgs
  limit <- fmap read $ fmap head getArgs
  pi <- Monte.estimatePi limit
  putStrLn $ show pi
