# DeviceCheckoutOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | **String** | The unique ID of the device intended for this `TerminalCheckout`. A list of `DeviceCode` objects can be retrieved from the /v2/devices/codes endpoint. Match a `DeviceCode.device_id` value with `device_id` to get the associated device code. | 
**skip_receipt_screen** | Option<**bool**> | Instructs the device to skip the receipt screen. Defaults to false. | [optional]
**collect_signature** | Option<**bool**> | Indicates that signature collection is desired during checkout. Defaults to false. | [optional]
**tip_settings** | Option<[**models::TipSettings**](TipSettings.md)> |  | [optional]
**show_itemized_cart** | Option<**bool**> | Show the itemization screen prior to taking a payment. This field is only meaningful when the checkout includes an order ID. Defaults to true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


