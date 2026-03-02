# DeviceCode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique id for this device code. | [optional][readonly]
**name** | Option<**String**> | An optional user-defined name for the device code. | [optional]
**code** | Option<**String**> | The unique code that can be used to login. | [optional][readonly]
**device_id** | Option<**String**> | The unique id of the device that used this code. Populated when the device is paired up. | [optional][readonly]
**product_type** | [**models::ProductType**](ProductType.md) |  | 
**location_id** | Option<**String**> | The location assigned to this code. | [optional]
**status** | Option<[**models::DeviceCodeStatus**](DeviceCodeStatus.md)> |  | [optional]
**pair_by** | Option<**String**> | When this DeviceCode will expire and no longer login. Timestamp in RFC 3339 format. | [optional][readonly]
**created_at** | Option<**String**> | When this DeviceCode was created. Timestamp in RFC 3339 format. | [optional][readonly]
**status_changed_at** | Option<**String**> | When this DeviceCode's status was last changed. Timestamp in RFC 3339 format. | [optional][readonly]
**paired_at** | Option<**String**> | When this DeviceCode was paired. Timestamp in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


