# DeletePaymentLinkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> |  | [optional]
**id** | Option<**String**> | The ID of the link that is deleted. | [optional]
**cancelled_order_id** | Option<**String**> | The ID of the order that is canceled. When a payment link is deleted, Square updates the the `state` (of the order that the checkout link created) to CANCELED. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


