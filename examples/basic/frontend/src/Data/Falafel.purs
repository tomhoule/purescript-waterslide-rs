module Data.Falafel where

import Data.Generic (
class Generic
)
import Data.Maybe (
Maybe
)

data Falafel = Falafel { basis :: FalafelBasis, parsley_percentage :: Int }

derive instance genericFalafel :: Generic Falafel

data FalafelBasis = FavaBean | Chickpea | Other (Maybe String)

derive instance genericFalafelBasis :: Generic FalafelBasis

data Meal = Meal { falafels :: (Array Falafel), with_salad :: Boolean }

derive instance genericMeal :: Generic Meal

