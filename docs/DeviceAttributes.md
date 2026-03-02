# DeviceAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::DeviceAttributesDeviceType**](DeviceAttributesDeviceType.md) |  | 
**manufacturer** | **String** | The maker of the device. | 
**model** | Option<**String**> | The specific model of the device. | [optional]
**name** | Option<**String**> | A seller-specified name for the device. | [optional]
**manufacturers_id** | Option<**String**> | The manufacturer-supplied identifier for the device (where available). In many cases, this identifier will be a serial number. | [optional]
**updated_at** | Option<**String**> | The RFC 3339-formatted value of the most recent update to the device information. (Could represent any field update on the device.) | [optional][readonly]
**version** | Option<**String**> | The current version of software installed on the device. | [optional]
**merchant_token** | Option<**String**> | The merchant_token identifying the merchant controlling the device. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


