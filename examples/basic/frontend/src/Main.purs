module Main where

import Prelude (Unit, bind, discard, pure, show, unit, ($))
import Data.Generic (gShow)
import Control.Monad.Eff (Eff)
import Control.Monad.Eff.Console (CONSOLE, log)
import Network.HTTP.Affjax (post, get, AJAX)
import Data.Argonaut.Generic.Decode (genericDecodeJson)
import Data.Argonaut.Generic.Encode (genericEncodeJson)
import Data.Argonaut.Generic.Aeson as Aeson
import Data.Either (Either, either)
import Control.Monad.Aff (Aff, liftEff', runAff)
import Control.Monad.Eff.Exception (Error, error)
import Control.Monad.Error.Class
import Data.Monoid ((<>))

import Data.Falafel (Meal)

mealSaga :: forall eff. Aff (ajax :: AJAX, console :: CONSOLE | eff) Meal
mealSaga = do
    res <- getMeal
    meal <- either (\s -> throwError $ error s) pure res
    postMeal meal
    pure meal

getMeal :: forall eff. Aff (ajax :: AJAX, console :: CONSOLE | eff) (Either String Meal)
getMeal = do
  res <- get "http://127.0.0.1:8077/meal"
  _ <- liftEff' $ log $ "Purescript side:\n" <> show res.response
  pure $ genericDecodeJson Aeson.options res.response

postMeal :: forall eff. Meal -> Aff (ajax :: AJAX, console :: CONSOLE | eff) Unit
postMeal meal = do
    res <- post "http://127.0.0.1:8077/meal" (genericEncodeJson Aeson.options meal)
    pure res.response

logSuccess :: forall eff. Meal -> Eff (console :: CONSOLE | eff) Unit
logSuccess meal = log (gShow meal)

logError :: forall e. Error -> Eff (console :: CONSOLE | e) Unit
logError err = log $ show err

main :: Eff (console :: CONSOLE, ajax :: AJAX) Unit
main = do
  log "Let's try to roundtrip"
  canceler <- runAff logError logSuccess mealSaga
  pure unit
