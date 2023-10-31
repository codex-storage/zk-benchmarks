
-- | Common types

{-# LANGUAGE PackageImports #-}
module Types where

--------------------------------------------------------------------------------

import Control.Monad

import Data.Char
import Data.Maybe
import Data.Fixed

import Data.Map (Map) 
import qualified Data.Map as Map

import System.FilePath

--------------------------------------------------------------------------------
-- * Benchmark phases

data Phase
  = Build
  | Setup
  | Witness
  | Run
  deriving (Eq,Ord,Show)

phaseBaseName :: Phase -> FilePath
phaseBaseName phase = case phase of
  Build   -> "build"  
  Setup   -> "setup"  
  Witness -> "witness"    
  Run     -> "run"

parsePhase :: String -> Maybe Phase
parsePhase str = case map toLower str of
  "build"   -> Just Build       
  "setup"   -> Just Setup       
  "witness" -> Just Witness         
  "run"     -> Just Run     
  _         -> Nothing

phaseScript :: Phase -> FilePath
phaseScript phase = phaseBaseName phase <.> "sh"

phaseLockFile :: Phase -> FilePath
phaseLockFile phase = phaseBaseName phase <.> "lock"

--------------------------------------------------------------------------------
-- * Parameters

newtype Params 
  = MkParams (Map String String)
  deriving (Eq,Show)

mkParams :: [(String,String)] -> Params
mkParams list = MkParams (Map.fromList list)

extendEnvWithParams :: Params -> [(String,String)] -> [(String,String)]
extendEnvWithParams (MkParams table) oldEnv = newEnv ++ filteredOld where
  filteredOld = filter (\pair -> not (fst pair `elem` newKeys)) oldEnv
  newKeys = map fst newEnv
  newEnv = [ ("ZKBENCH_" ++ key, value) | (key,value) <- Map.toList table ]

--------------------------------------------------------------------------------
-- * Tags

-- | Tags are used to select subsets of the benchmarks.
--
-- A tag can be fixed constant, say @"Groth16"@, or a reference to a
-- parameter, for example @"$PROVER".
--
data Tag
  = FixedTag String
  | ParamTag String
  deriving (Eq,Show)

--------------------------------------------------------------------------------
-- * Time

newtype Seconds a
  = MkSeconds a
  deriving (Eq,Ord,Show)

fromSeconds :: Seconds a -> a
fromSeconds (MkSeconds x) = x

--------------------------------------------------------------------------------
-- * Benchmark config

data Benchmark = MkBenchmark
  { _benchDir        :: FilePath
  , _benchTimeout    :: Seconds Int
  , _benchRerunFrom  :: Phase 
  , _benchPhases     :: [Phase]
  , _benchParams     :: Map String [String]
  , _benchTags       :: [Tag]
  , _benchName       :: Maybe String
  , _benchAuthor     :: Maybe String
  , _benchComment    :: Maybe String
  }
  deriving Show

dummyBenchmark :: Benchmark
dummyBenchmark = MkBenchmark
  { _benchDir        = "."
  , _benchTimeout    = MkSeconds 60
  , _benchRerunFrom  = Run
  , _benchPhases     = [Run]
  , _benchParams     = Map.empty
  , _benchTags       = []
  , _benchName       = Nothing
  , _benchAuthor     = Nothing
  , _benchComment    = Nothing
  }

--------------------------------------------------------------------------------
-- * Results

data Result = MkResult
  { _resParams  :: !Params
  , _resPhase   :: !Phase  
  , _resTags    :: [Tag]
  , _resAvgTime :: !(Seconds Double)
  }
  deriving Show

--------------------------------------------------------------------------------
-- * Misc

quote :: String -> String
quote str = "`" ++ str ++ "`"

--------------------------------------------------------------------------------
