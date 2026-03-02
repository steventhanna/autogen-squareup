# ModifierLocationOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_id** | Option<**String**> | The ID of the `Location` object representing the location. This can include a deactivated location. | [optional]
**price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**sold_out** | Option<**bool**> | Indicates whether the modifier is sold out at the specified location or not. As an example, for cheese (modifier) burger (item), when the modifier is sold out, it is the cheese, but not the burger, that is sold out. The seller can manually set this sold out status. Attempts by an application to set this attribute are ignored. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


