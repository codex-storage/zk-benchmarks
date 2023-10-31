
-- | Parsing the @bench.cfg@ files.
--
-- Example config file:
--
-- > name: "Poseidon2 Groth16 benchmarks"
-- > timeout: 300
-- > rerunFrom: build 
-- > params:
-- >   [ PROVER:     [ snarkjs, rapidsnark ]
-- >   , INPUT_SIZE: [ 256, 512, 1024, 2048 ]
-- >   , WHICH:      [ hash_sponge, hash_sponge_rate2, hash_merkle ]
-- >   ]
-- > tags: Groth16, Poseidon2, $PROVER
--

{-# OPTIONS_GHC -Wno-overlapping-patterns #-}
{-# LANGUAGE PackageImports #-}
module Parser where

--------------------------------------------------------------------------------

import Data.Char
import Data.Maybe
-- import Data.List

import qualified Data.Map as Map

import Control.Monad
import Control.Applicative

import "parsec1" Text.ParserCombinators.Parsec

import Types

--------------------------------------------------------------------------------

exampleConfigString :: String
exampleConfigString = unlines 
  [ "name: \"Poseidon2 Groth16 benchmarks\"  "
  , "timeout: 300"
  , "rerunFrom: build  "
  , "params:"
  , "  [ PROVER:     [ snarkjs, \"rapidsnark\" ]"
  , "  , INPUT_SIZE: [ 256, 512, 1024,   "
  , "                    2048 ]"
  , "  , WHICH:      [ hash_sponge, hash_sponge_rate2, hash_merkle ]"
  , "  ]"
  , "tags: Groth16, Poseidon2, $PROVER, \"stringy tag\"  "
  , "comment:  "
  , "  foo"
  , "  bar"
  , "  baz"
  , "author: \"X. Middlename Y.\""
  ]

--------------------------------------------------------------------------------

data CfgField
  = Field_name
  | Field_timeout   
  | Field_rerun   
  | Field_params   
  | Field_tags   
  | Field_author   
  | Field_comment
  deriving Show

recognizeField :: String -> Maybe CfgField
recognizeField s = case filter (/='_') (map toLower s) of
  "name"      -> Just Field_name
  "timeout"   -> Just Field_timeout
  "rerun"     -> Just Field_rerun  
  "rerunfrom" -> Just Field_rerun  
  "params"    -> Just Field_params 
  "tags"      -> Just Field_tags   
  "author"    -> Just Field_author 
  "comment"   -> Just Field_comment
  "comments"  -> Just Field_comment
  _           -> Nothing

--------------------------------------------------------------------------------

naturalP :: Parser Integer
naturalP = do
  xs <- many1 digit
  return (read xs)

numberP :: Parser Integer
numberP = naturalP

identP :: Parser String
identP = do
  c  <- letter <|> oneOf "_"
  cs <- many (alphaNum <|> oneOf "_")
  return (c:cs)

singleWordP :: Parser String
singleWordP = do
  c  <- letter <|> oneOf "_"
  cs <- many (alphaNum <|> oneOf "_-")
  return (c:cs)

quotedStringP :: Parser String
quotedStringP = do
  char '"'
  xs <- many (noneOf "\"\n\r")
  char '"'
  return xs

tagP :: Parser Tag
tagP = (FixedTag <$> quotedStringP) <|> nakedTagP

nakedTagP :: Parser Tag
nakedTagP = do
  c  <- letter <|> oneOf "_$"
  cs <- many (alphaNum <|> oneOf "_-+!?@#%^/&*=:")
  return $ case c of 
    '$' -> ParamTag cs
    _   -> FixedTag (c:cs)

--------------------------------------------------------------------------------

-- | A list delimited by @[@ and @]@
listP :: Parser a -> Parser [a]
listP userP = (char '[' >> spaces >> start) where
  finish = do
    char ']' 
    return []
  start  = finish <|> do 
    x  <- userP ; spaces
    xs <- continue
    return (x:xs)
  continue = finish <|> do
    char ','    ; spaces 
    x  <- userP ; spaces
    xs <- continue
    return (x:xs)

-- | A list without delimiters
nakedListP :: Parser a -> Parser [a]
nakedListP userP = start where
  start = do
    x  <- userP ; onlySpaces
    xs <- continue
    return (x:xs)
  continue = (newlineP >> return []) <|> do
    char ','    ; spaces 
    x  <- userP ; onlySpaces
    xs <- continue
    return (x:xs)

--------------------------------------------------------------------------------

postEOF :: Parser a -> Parser a
postEOF userP = do
  x <- userP
  spaces
  eof
  return x

postNewline :: Parser a -> Parser a
postNewline userP = do
  x <- userP
  onlySpaces
  newlineP
  return x

postSpaces :: Parser a -> Parser a
postSpaces userP = do
  x <- userP
  spaces
  return x

--------------------------------------------------------------------------------

onlySpaces :: Parser ()
onlySpaces = void $ many (oneOf " \t")

newlineP :: Parser ()
newlineP  =  try (void $ string "\r\n") 
         <|>     (void $ oneOf  "\r\n")
         <|> eof

singleLineP :: Parser String
singleLineP = quotedStringP <|> many (noneOf "\r\n")

multiLineP_ :: Parser String
multiLineP_ = unlines <$> multiLineP

multiLineP :: Parser [String]
multiLineP = loop where
  loop = do
    l  <- tillEndOfLine
    ls <- indented
    return (l:ls)
  indented = do
    newlineP 
    continue <|> return []
  continue = do
    _ <- many1 (oneOf " \t") 
    loop
  tillEndOfLine = many (noneOf "\r\n")

--------------------------------------------------------------------------------

phaseP :: Parser Phase
phaseP = do
  s <- identP
  case (map toLower s) of 
    "build"   -> return Build
    "setup"   -> return Setup
    "witness" -> return Witness
    "run"     -> return Run
    _         -> fail ("unknown phase " ++ quote s)

tagsP :: Parser [Tag]
tagsP = listP tagP <|> nakedListP tagP

--------------------------------------------------------------------------------

simpleValP :: Parser Value
simpleValP  =  (StringV <$> (quotedStringP <|> singleWordP)) 
           <|> (NumberV <$> numberP)

type ParamRange = KeyVal String [Value]

paramP :: Parser ParamRange
paramP = do
  key <- identP ; spaces
  char ':'      ; spaces
  vals <- listP simpleValP
  return (MkKeyVal key vals)

paramsP :: Parser [ParamRange]
paramsP = listP paramP

--------------------------------------------------------------------------------

data KeyVal k a
  = MkKeyVal k a
  deriving (Eq,Show)

data Value 
  = StringV String
  | NumberV Integer
  | PhaseV  Phase
  | TagV    Tag
  | ListV   [Value]
  | KeyValV (KeyVal String Value)
  deriving (Eq,Show)

paramRangeToValue :: ParamRange -> Value
paramRangeToValue (MkKeyVal key list) = KeyValV (MkKeyVal key (ListV list))

type Entry = KeyVal CfgField Value

entryP :: Parser Entry
entryP = 
  postSpaces $ do
    key <- identP ; spaces
    char ':'      ; spaces
    (fld,val) <- case recognizeField key of
      Nothing  -> fail ("invalid configuration key " ++ quote key)
      Just fld -> do
        rhs <- fieldValueP fld
        return (fld,rhs)
    return (MkKeyVal fld val)
  where

fieldValueP :: CfgField -> Parser Value
fieldValueP fld = case fld of
  Field_name    -> StringV <$> singleLineP
  Field_timeout -> NumberV <$> numberP
  Field_rerun   -> PhaseV  <$> phaseP
  Field_params  -> ListV   <$> (map paramRangeToValue <$> paramsP)
  Field_tags    -> ListV   <$> (map TagV              <$> tagsP  )
  Field_author  -> StringV <$> singleLineP
  Field_comment -> StringV <$> multiLineP_

configP :: Parser [Entry]
configP = spaces >> postEOF (many1 entryP)

--------------------------------------------------------------------------------

fromTagV :: Value -> Maybe Tag
fromTagV (TagV tag) = Just tag
fromTagV _          = Nothing

fromSimpleV :: Value -> Maybe String
fromSimpleV (StringV s) = Just s
fromSimpleV (NumberV x) = Just (show x)
fromSimpleV _           = Nothing

fromParamV :: Value -> Maybe (String,[String])
fromParamV (KeyValV kv) = case kv of
  MkKeyVal s (ListV list)  ->  Just (s, mapMaybe fromSimpleV list)
  MkKeyVal s  value        ->  (\y -> (s,[y])) <$> fromSimpleV value
fromParamV _ = Nothing

-- fromParamsV :: Value -> Maybe [(String,[String])]
-- fromParamsV (ListV list) = Just (catMaybes $ fromParamV list)
-- fromParamsV _            = Nothing

--------------------------------------------------------------------------------

type ErrM a = Either String a 

err :: String -> ErrM a
err = Left

entriesToBenchmark :: [Entry] -> ErrM Benchmark
entriesToBenchmark = entriesToBenchmark' dummyBenchmark

entriesToBenchmark' :: Benchmark -> [Entry] -> ErrM Benchmark
entriesToBenchmark' oldBenchmark = foldM handle oldBenchmark where

  handle :: Benchmark -> KeyVal CfgField Value -> ErrM Benchmark
  handle old (MkKeyVal key val) = case key of

    Field_name    -> case val of
      StringV s     -> return $ old { _benchName = Just s }
      _             -> err "unexpected type for field `name`"

    Field_timeout -> case val of
      NumberV t     -> return $ old { _benchTimeout = MkSeconds (fromInteger t) }
      _             -> err "unexpected type for field `timeout`"

    Field_rerun   -> case val of
      PhaseV p      -> return $ old { _benchRerunFrom = p }
      _             -> err "unexpected type for field `rerun_from`"

    Field_params  -> case val of
      ListV list    -> return $ old { _benchParams = Map.fromList (mapMaybe fromParamV list) }
      _             -> err "unexpected type for field `rparams`"

    Field_tags    -> case val of
      ListV list    -> return $ old { _benchTags = mapMaybe fromTagV list }
      _             -> err "unexpected type for field `tags`"

    Field_author  -> case val of
      StringV s     -> return $ old { _benchAuthor = Just s }
      _             -> err "unexpected type for field `author`"

    Field_comment -> case val of
      StringV s     -> return $ old { _benchComment = Just s }
      _             -> err "unexpected type for field `comment`"

    _ -> err $ "unknown field " ++ show key

--------------------------------------------------------------------------------

parseConfig :: FilePath -> String -> ErrM Benchmark
parseConfig fpath str = case parse configP fpath str of
  Left  err     -> Left (show err)
  Right entries -> entriesToBenchmark entries 

parseConfig_ :: String -> ErrM Benchmark
parseConfig_ = parseConfig "<config>"

--------------------------------------------------------------------------------
