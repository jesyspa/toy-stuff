import Data.Function (on)
import Data.List (sortBy, intercalate)
{-
Program for printing the shortest way of saying a particular phrase in Lojban.  This only takes syntactically
equivalent rewrites into account; there may be shorter semantically equivalent phrases.

The idea is as follows: a bridi is a selbri, with zero or more attached sumti.  Some of the sumti are attached through
explicitly marked places, while others may be attached through tenses or BAI.

The grammar is as follows:
BRIDI -> MARKED_SUMTI* [cu] TANRU MARKED_SUMTI*
MARKED_SUMTI -> [FA/BAI] SUMTI
SUMTI -> ko'X
SUMTI -> lo TANRU [ku]
TANRU -> SELBRI TANRU?
SELBRI -> brodX
SELBRI -> su'u BRIDI [kei]
FA -> fX
BAI -> do'e

This is much less than the full Lojban grammar: note the absence of BE, GOI, and NOI in particular.
-}

data Bridi = Bridi Tanru [Sumti] deriving (Eq, Ord, Read, Show)
data Tanru = Tanru [Selbri] deriving (Eq, Ord, Read, Show)
data Selbri = Broda String | Abstraction Bridi deriving (Eq, Ord, Read, Show)
data Sumti = Koha String | Lo Tanru deriving (Eq, Ord, Read, Show)

data Phrase = Phrase { phrase :: [String], syllables :: Int } deriving (Eq, Ord, Read, Show)

(|+|) :: Phrase -> Phrase -> Phrase
Phrase p1 s1 |+| Phrase p2 s2 = Phrase (p1 ++ p2) (s1 + s2)

pEmpty = Phrase [] 0
pOneSyllable s = Phrase [s] 1
pTwoSyllable s = Phrase [s] 2

shortestPhrase :: [Phrase] -> Phrase
shortestPhrase = head . sortBy (compare `on` syllables)

-- We use these to indicate whether a particular phrase needs to be terminated with respect to these; that is, whether 
type BridiTermination = Bool
type SumtiTermination = Bool
type AllTermination = (BridiTermination, SumtiTermination)

class LevelShow t where
    -- Generate a phrase with the given level of termination required.
    showAt :: t -> AllTermination -> Phrase

instance LevelShow Sumti where
    -- Note that we assume that ko'a really is two syllables, so e.g. {mi} would give the wrong syllable count here.
    -- However, since the word has to appear in the output exactly once, this does not matter for chosing the optimal
    -- structure.
    showAt (Koha s) _ = pTwoSyllable s
    showAt (Lo tanru) t@(_, st) = pOneSyllable "lo" |+| best
        where with_ku = showAt tanru (False, True) |+| pOneSyllable "ku"
              without_ku = showAt tanru t
              best = if st then with_ku else shortestPhrase [with_ku, without_ku]

-- Clunky, but works.  First value outside the list is what to give non-final elements, second is what to give
-- the final element.
showMany :: [AllTermination -> Phrase] -> AllTermination -> AllTermination -> Phrase
showMany [] _ _ = pEmpty
showMany [s] _ t = s t
showMany (s:st) ti tf = s ti |+| showMany st ti tf

instance LevelShow Tanru where
    showAt (Tanru tanru) t = showMany (map showAt tanru) (True, True) t

instance LevelShow Bridi where
    showAt (Bridi tanru sumti) t = shortestPhrase [ go sa sb | n <- [0..length sumti], let (sa, sb) = splitAt n sumti' ]
        where sumti' = map showAt sumti
              go sa sb =
                let with_cu_sa = showMany sa (False, True) (False, False) |+| pOneSyllable "cu"
                    without_cu_sa = showMany sa (False, True) (True, True)
                    shown_tanru = showAt tanru (if length sb == 0 then t else (True, False))
                    shown_sb = showMany sb (False, True) t
                    shown_sb' = (if length sa == 0 && length sb > 0 then pOneSyllable "fa" else pEmpty) |+| shown_sb
                in shortestPhrase [with_cu_sa, without_cu_sa] |+| shown_tanru |+| shown_sb'

instance LevelShow Selbri where
    showAt (Broda s) _ = pTwoSyllable s
    showAt (Abstraction bridi) t@(bt, _) = pTwoSyllable "su'u" |+| best
        where with_kei = showAt bridi (True, False) |+| pOneSyllable "kei"
              without_kei = showAt bridi t
              best = if bt then with_kei else shortestPhrase [with_kei, without_kei]

showBase :: LevelShow s => s -> String
showBase s = intercalate " " $ phrase $ showAt s (False, False)

eKoh c = Koha $ "ko'" ++ [c]
eBrod c = Broda $ "brod" ++ [c]
eBridi ts ss = Bridi (Tanru ts) ss
eSumti ts = Lo (Tanru ts)
eAbs ts ss = Abstraction $ eBridi ts ss

example = eBridi [eBrod 'a', eAbs [eBrod 'e'] [eKoh 'a']] [eSumti [eAbs [eBrod 'i'] [eKoh 'e', eSumti [eBrod 'o']]], eKoh 'e']
