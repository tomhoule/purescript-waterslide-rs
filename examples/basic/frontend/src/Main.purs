module Main where

import Prelude
import Data.Generic (gShow)
import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Console (CONSOLE, log)
import Network.HTTP.Affjax (get, AJAX)
import Data.Argonaut.Generic.Decode (genericDecodeJson)
import Data.Either (Either(..))
import Control.Monad.Aff
import Control.Monad.Eff.Exception

import Data.Falafel (Meal)
import Options as O

getMeal :: forall eff. Aff (ajax :: AJAX, console :: CONSOLE | eff) (Either String Meal)
getMeal = do
  res <- get "http://127.0.0.1:8077/meal"
  _ <- liftEff' $ log $ show res.response
  pure $ genericDecodeJson O.options res.response

logSuccess :: forall eff. Either String Meal -> Eff (console :: CONSOLE | eff) Unit
logSuccess (Right meal) = log (gShow meal)
logSuccess (Left err) = log err

logError :: forall e. Error -> Eff (console :: CONSOLE | e) Unit
logError err = log $ show err

main :: Eff (console :: CONSOLE, ajax :: AJAX) Unit
main = do
  log "Let's try to roundtrip"
  canceler <- runAff logError logSuccess getMeal
  pure unit
