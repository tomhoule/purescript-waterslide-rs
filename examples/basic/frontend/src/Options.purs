module Options where

import Debug.Trace
import Data.Maybe (Maybe(..))
import Data.Either
import Data.Argonaut.Core (Json, foldJsonArray, foldJson, jsonNull, isNull, jsonSingletonArray, jsonTrue, jsonSingletonObject)
import Data.Argonaut.Generic.Options (Options(..))
import Data.Argonaut.Generic.Aeson as Argonaut
import Data.Argonaut.Generic.Util
import Data.List (List, fromFoldable)
import Data.Array (filter)
import Partial.Unsafe (unsafePartial)
import Data.Array.Partial as Unsafe
import Prelude (bind, pure, ($), unit, (<<<), (==), map)
import Data.Generic (class Generic, GenericSpine(SProd, SString), DataConstructor, GenericSignature(SigProd, SigUnit), class Generic)
import Data.Argonaut.Generic.Encode (genericUserEncodeJson')
import Data.Argonaut.Generic.Decode (genericUserDecodeJson')

options :: Options
options = case Argonaut.options of
               Options opts -> Options opts { userDecoding = userDecoding , userEncoding = userEncoding, flattenContentsArray = true }

userEncoding :: Options -> GenericSignature -> GenericSpine -> Maybe Json
userEncoding opts sig json = encodeMaybe opts sig json

encodeMaybe :: Options -> GenericSignature -> GenericSpine -> Maybe Json
encodeMaybe opts (SigProd "Data.Maybe.Maybe" sigArr) (SProd "Data.Maybe.Just" [elem]) =
    pure $ genericUserEncodeJson' opts valSig val
      where
          valSig = getSigFromUnaryConstructor sigArr "Data.Maybe.Just"
          val = elem unit
encodeMaybe opts (SigProd "Data.Maybe.Maybe" sigArr) (SProd "Data.Maybe.Nothing" _) =
    pure jsonNull
encodeMaybe _ _ _ = Nothing

userDecoding :: Options -> GenericSignature -> Json -> Maybe (Either String GenericSpine)
userDecoding opts sig json = decodeMaybe opts sig json

decodeMaybe :: Options -> GenericSignature -> Json -> Maybe (Either String GenericSpine)
decodeMaybe opts (SigProd "Data.Maybe.Maybe" sigArr) json | isNull json = spy $ pure $ Right $ SProd "Data.Maybe.Nothing" []
decodeMaybe opts (SigProd "Data.Maybe.Maybe" sigArr) json =
    spy $ Just $ map (\e -> SProd "Data.Maybe.Just" [\u -> e]) $ genericUserDecodeJson' opts sig json
    where
        sig = spy $ getSigFromUnaryConstructor sigArr "Data.Maybe.Just"
decodeMaybe _ _ _ = Nothing

getInsideJust :: Array DataConstructor -> GenericSignature
getInsideJust arr = sig
    where
        sig = ((\a -> unsafeHead $ a.sigValues) $ just) unit
        just = unsafeHead <<< filter ((\a -> a == "Data.Maybe.Just") <<< _.sigConstructor) $ arr

getSigFromUnaryConstructor :: Array DataConstructor -> String -> GenericSignature
getSigFromUnaryConstructor arr name = unsafeHead $ getSigsFromConstructor arr name

getSigsFromConstructor :: Array DataConstructor -> String -> Array GenericSignature
getSigsFromConstructor arr name =
    let constr = unsafeHead <<< filter ((_ == name) <<< _.sigConstructor) $ arr
    in map (_ $ unit) constr.sigValues

arrToList :: forall a. Array a -> List a
arrToList = fromFoldable

unsafeHead :: forall a. Array a -> a
unsafeHead = unsafePartial Unsafe.head

unsafeTail :: forall a. Array a -> Array a
unsafeTail = unsafePartial Unsafe.tail
