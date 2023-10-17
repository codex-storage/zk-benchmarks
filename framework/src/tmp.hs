
module Main where

import Runner

--------------------------------------------------------------------------------

bench1 = MkBenchmark
  { _benchDir        = "../../hash/snark/bench/Poseidon2"
  , _benchTimeout    = MkSeconds 30
  , _benchRerunFrom  = Build
  , _benchPhases     = [Build,Setup,Witness,Run]
  , _benchParams     = mkParams 
       [ ("INPUT_SIZE" , "64"         )
       , ("WHICH"      , "hash_sponge")
       , ("PROVER"     , "snarkjs"    )
       ]
  }

main = runBenchmark False bench1