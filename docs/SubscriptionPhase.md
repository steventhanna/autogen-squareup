# SubscriptionPhase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | The Square-assigned ID of the subscription phase. This field cannot be changed after a `SubscriptionPhase` is created. | [optional]
**cadence** | [**models::SubscriptionCadence**](SubscriptionCadence.md) |  | 
**periods** | Option<**i32**> | The number of `cadence`s the phase lasts. If not set, the phase never ends. Only the last phase can be indefinite. This field cannot be changed after a `SubscriptionPhase` is created. | [optional]
**recurring_price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**ordinal** | Option<**i64**> | The position this phase appears in the sequence of phases defined for the plan, indexed from 0. This field cannot be changed after a `SubscriptionPhase` is created. | [optional]
**pricing** | Option<[**models::SubscriptionPricing**](SubscriptionPricing.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


