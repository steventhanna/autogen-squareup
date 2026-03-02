# AdditionalRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_id** | **String** | The location ID for a recipient (other than the merchant) receiving a portion of this tender. | 
**description** | Option<**String**> | The description of the additional recipient. | [optional]
**amount_money** | [**models::Money**](Money.md) |  | 
**receivable_id** | Option<**String**> | The unique ID for the RETIRED `AdditionalRecipientReceivable` object. This field should be empty for any `AdditionalRecipient` objects created after the retirement. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


