
-- | Collect the different benchmarks from the file system
--
-- Essentially we look for subdirectories where a `bench.cfg` file is present,
-- and try to parse them, and the filter based on tags.
--

module Collect where

--------------------------------------------------------------------------------

import Control.Monad
import Control.Applicative
import Control.Exception

import Data.IORef

import System.FilePath
import System.Directory
import System.IO

import Types
import Parser

--------------------------------------------------------------------------------

collectBenches :: FilePath -> IO [Benchmark]
collectBenches rootDir0 = 
  do
    rootDir <- canonicalizePath rootDir0
    ref <- newIORef [] :: IO (IORef [Benchmark])
    walk ref rootDir
    readIORef ref

  where

    walk :: IORef [Benchmark] -> FilePath -> IO ()
    walk ref dir = do
      check ref dir
      list <- listDirectory dir
      forM_ list $ \fn -> do
        let full = dir </> fn
        b <- doesDirectoryExist full
        when b (walk ref full)

    check :: IORef [Benchmark] -> FilePath -> IO ()
    check ref dir = do
      let fpath = dir </> "bench.cfg" 
      b <- doesFileExist fpath
      when b $ do
        putStr $ "found `" ++ fpath ++ "`"
        ei <- trySome $ do
          text <- readFile fpath
          case parseConfig fpath text of
            Left  err  -> throw (ParseError err)
            Right cfg0 -> do
              let cfg = cfg0 { _benchDir = dir } 
              modifyIORef ref (cfg:) 
              return ()
        case ei of
          Left err -> do
            putStrLn " - parsing FAILED!"
            return ()
          Right () -> do
            putStrLn " - OK."
            return ()

--------------------------------------------------------------------------------

data ParseError 
  = ParseError String
  deriving (Show)

instance Exception ParseError

trySome :: IO a -> IO (Either SomeException a)
trySome = try

--------------------------------------------------------------------------------

