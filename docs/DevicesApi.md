# \DevicesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_device_code**](DevicesApi.md#create_device_code) | **POST** /v2/devices/codes | CreateDeviceCode
[**get_device**](DevicesApi.md#get_device) | **GET** /v2/devices/{device_id} | GetDevice
[**get_device_code**](DevicesApi.md#get_device_code) | **GET** /v2/devices/codes/{id} | GetDeviceCode
[**list_device_codes**](DevicesApi.md#list_device_codes) | **GET** /v2/devices/codes | ListDeviceCodes
[**list_devices**](DevicesApi.md#list_devices) | **GET** /v2/devices | ListDevices



## create_device_code

> models::CreateDeviceCodeResponse create_device_code(create_device_code_request)
CreateDeviceCode

Creates a DeviceCode that can be used to login to a Square Terminal device to enter the connected terminal mode.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_device_code_request** | [**CreateDeviceCodeRequest**](CreateDeviceCodeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateDeviceCodeResponse**](CreateDeviceCodeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device

> models::GetDeviceResponse get_device(device_id)
GetDevice

Retrieves Device with the associated `device_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | The unique ID for the desired `Device`. | [required] |

### Return type

[**models::GetDeviceResponse**](GetDeviceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_code

> models::GetDeviceCodeResponse get_device_code(id)
GetDeviceCode

Retrieves DeviceCode with the associated ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The unique identifier for the device code. | [required] |

### Return type

[**models::GetDeviceCodeResponse**](GetDeviceCodeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_device_codes

> models::ListDeviceCodesResponse list_device_codes(cursor, location_id, product_type, status)
ListDeviceCodes

Lists all DeviceCodes associated with the merchant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for your original query.  See [Paginating results](https://developer.squareup.com/docs/working-with-apis/pagination) for more information. |  |
**location_id** | Option<**String**> | If specified, only returns DeviceCodes of the specified location. Returns DeviceCodes of all locations if empty. |  |
**product_type** | Option<[**ProductType**](.md)> | If specified, only returns DeviceCodes targeting the specified product type. Returns DeviceCodes of all product types if empty. |  |
**status** | Option<[**DeviceCodeStatus**](.md)> | If specified, returns DeviceCodes with the specified statuses. Returns DeviceCodes of status `PAIRED` and `UNPAIRED` if empty. |  |

### Return type

[**models::ListDeviceCodesResponse**](ListDeviceCodesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_devices

> models::ListDevicesResponse list_devices(cursor, sort_order, limit, location_id)
ListDevices

List devices associated with the merchant. Currently, only Terminal API devices are supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. See [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination) for more information. |  |
**sort_order** | Option<[**SortOrder**](.md)> | The order in which results are listed. - `ASC` - Oldest to newest. - `DESC` - Newest to oldest (default). |  |
**limit** | Option<**i32**> | The number of results to return in a single page. |  |
**location_id** | Option<**String**> | If present, only returns devices at the target location. |  |

### Return type

[**models::ListDevicesResponse**](ListDevicesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

