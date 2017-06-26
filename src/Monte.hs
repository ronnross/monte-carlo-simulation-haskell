{-# LANGUAGE BangPatterns #-}
module Monte where

import Data.List (map, length, foldl')
import System.Random (RandomGen, Random)
import qualified System.Random as R

type Point = (Double, Double)

instance (Random a, Random b) => Random (a, b) where
--   -- RandomGen g => (a, a) -> g -> (a, g)
  randomR ((loa, lob), (hia, hib)) gen =
    let
      (first, newGen) = R.randomR (loa, hia) gen
      (second, newGen') = R.randomR (lob, hib) newGen
    in
      ((first, second), newGen')

--   -- RandomGen g => g -> (a, g)
  random gen =
    let
      (first, newGen) = R.random gen
      (second, newGen') = R.random newGen
    in
      ((first, second), newGen')

-- fmap (take 10) (fmap getRandom getStdGen)
getRandom :: RandomGen g => g -> [Point]
getRandom gen =
  let (num, newGen) = R.random gen
  in
    num : getRandom newGen

estimatePi :: Int -> IO Double
estimatePi limit = fmap (* 4) $ fmap averageWeights' $ fmap (take limit) (fmap getRandom R.getStdGen)

weight :: Point -> Double
weight (x, y) =
  let distance = sqrt $ x * x + y * y
  in
    if distance <= 1 then 1 else 0

averageWeights :: [Point] -> Double
averageWeights points =
  let weightedValues = map weight points
  in
    sum weightedValues / fromIntegral (length weightedValues)

averageWeights' :: [Point] -> Double
averageWeights' points =
  let (!length, !sum) = foldl' (\ (!len, !sum) point -> (len + 1, sum + weight point)) (0, 0) points
  in sum / length

approximatePi :: [Point] -> Double
approximatePi points = averageWeights points * 4

approximatePi' :: [Point] -> Double
approximatePi' points = averageWeights' points * 4
