
-- | Run a single benchmark

{-# LANGUAGE PackageImports #-}
module Runner where

--------------------------------------------------------------------------------

import Control.Monad

import Data.Char
import Data.Maybe
import Data.Fixed

import Data.Map (Map) 
import qualified Data.Map as Map

import Text.Printf

import System.IO
import System.FilePath
import System.Directory
import System.Environment
import System.Process

import "time" Data.Time.Clock
import "time" Data.Time.Clock.System

import Types

--------------------------------------------------------------------------------

createLockFile :: FilePath -> IO ()
createLockFile fpath = do
  h <- openBinaryFile fpath WriteMode 
  hClose h

--------------------------------------------------------------------------------

runBenchmark :: Bool -> Benchmark -> IO [Result]
runBenchmark rerunAll bench = do
  origEnv <- getEnvironment
  origDir <- getCurrentDirectory 

  path <- canonicalizePath (_benchDir bench)
  setCurrentDirectory path
  getCurrentDirectory >>= putStrLn

  mbs <- forM (_benchPhases bench) $ \phase -> do
    let script = phaseScript phase
    b <- doesFileExist script
    if (not b)
      then do
        putStrLn ("WARNING: benchmark script " ++ quote script ++ " does not exist!")
        return Nothing
      else do
        let extendedEnv = extendEnvWithParams (_benchParams bench) origEnv
        let cp = (proc "bash" [script]) { env = Just extendedEnv }
        mbElapsed <- runSinglePhase 
          (rerunAll || phase >= _benchRerunFrom bench) 
          phase
          (_benchTimeout bench) 
          cp
        let f secs = MkResult (_benchParams bench) phase secs
        return $ fmap f mbElapsed

  return (catMaybes mbs)

--------------------------------------------------------------------------------

-- | runs a process
runCreateProcess :: CreateProcess -> IO ()
runCreateProcess cp = withCreateProcess cp $ \stdin stdout stderr handle -> do
  waitForProcess handle
  return ()

-- | runs a process and measures the elapsed time (in seconds)
run1 :: CreateProcess -> IO Double
run1 cp = do
  before <- getSystemTime 
  runCreateProcess cp 
  after  <- getSystemTime
  let diff = diffUTCTime (systemToUTCTime after) (systemToUTCTime before) 
  return (realToFrac diff)

-- | Runs a single phase (eg. @build@ or @run@)
runSinglePhase :: Bool -> Phase -> Seconds Int -> CreateProcess -> IO (Maybe (Seconds Double))
runSinglePhase alwaysRerun phase timeout cp = do
  let lockfile = "build" </> phaseLockFile phase
  b <- doesFileExist lockfile
  if (alwaysRerun || not b)
    then do
      putStrLn $ "running phase " ++ show phase
      (n,avg) <- runSinglePhaseAlways phase timeout cp
      createLockFile lockfile
      printf "average wall-clock time (from %d runs) for phase `%s` = %.5f seconds\n" n (show phase) (fromSeconds avg)
      return (Just avg)
    else do
      putStrLn $ "skipping phase " ++ show phase
      return Nothing

-- | Runs a single phase unconditionally; in case of the "Run" phase, possibly
-- several times to get a more precise measurement
runSinglePhaseAlways :: Phase -> Seconds Int -> CreateProcess -> IO (Int,Seconds Double)
runSinglePhaseAlways phase (MkSeconds targetTime) cp = do
  elapsed1 <- run1 cp
  let n = if phase == Run 
            then min 10 (round (fromIntegral targetTime / elapsed1)) :: Integer
            else 1
  elapsedRest <- forM [2..n] $ \_ -> run1 cp
  let avg = sum (elapsed1 : elapsedRest) / fromIntegral n :: Double
  return (fromInteger n, MkSeconds avg)

--------------------------------------------------------------------------------
