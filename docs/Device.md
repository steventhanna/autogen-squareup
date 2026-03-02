# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A synthetic identifier for the device. The identifier includes a standardized prefix and is otherwise an opaque id generated from key device fields. | [optional][readonly]
**attributes** | [**models::DeviceAttributes**](DeviceAttributes.md) |  | 
**components** | Option<[**Vec<models::Component>**](Component.md)> | A list of components applicable to the device. | [optional]
**status** | Option<[**models::DeviceStatus**](DeviceStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


