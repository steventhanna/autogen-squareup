# DeviceComponentDetailsWiFiDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | A boolean to represent whether the WiFI interface is currently active. | [optional]
**ssid** | Option<**String**> | The name of the connected WIFI network. | [optional]
**ip_address_v4** | Option<**String**> | The string representation of the device’s IPv4 address. | [optional]
**secure_connection** | Option<**String**> | The security protocol for a secure connection (e.g. WPA2). None provided if the connection is unsecured. | [optional]
**signal_strength** | Option<[**models::DeviceComponentDetailsMeasurement**](DeviceComponentDetailsMeasurement.md)> |  | [optional]
**mac_address** | Option<**String**> | The mac address of the device in this network. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


