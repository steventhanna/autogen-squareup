# \ChannelsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_retrieve_channels**](ChannelsApi.md#bulk_retrieve_channels) | **POST** /v2/channels/bulk-retrieve | BulkRetrieveChannels
[**list_channels**](ChannelsApi.md#list_channels) | **GET** /v2/channels | ListChannels
[**retrieve_channel**](ChannelsApi.md#retrieve_channel) | **GET** /v2/channels/{channel_id} | RetrieveChannel



## bulk_retrieve_channels

> models::BulkRetrieveChannelsResponse bulk_retrieve_channels(bulk_retrieve_channels_request)
BulkRetrieveChannels



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_retrieve_channels_request** | [**BulkRetrieveChannelsRequest**](BulkRetrieveChannelsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkRetrieveChannelsResponse**](BulkRetrieveChannelsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channels

> models::ListChannelsResponse list_channels(reference_type, reference_id, status, cursor, limit)
ListChannels



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_type** | Option<[**ReferenceType**](.md)> | Type of reference associated to channel |  |
**reference_id** | Option<**String**> | id of reference associated to channel |  |
**status** | Option<[**ChannelStatus**](.md)> | Status of channel |  |
**cursor** | Option<**String**> | Cursor to fetch the next result |  |
**limit** | Option<**i32**> | Maximum number of results to return. When not provided the returned results will be cap at 100 channels. |  |

### Return type

[**models::ListChannelsResponse**](ListChannelsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_channel

> models::RetrieveChannelResponse retrieve_channel(channel_id)
RetrieveChannel



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | A channel id | [required] |

### Return type

[**models::RetrieveChannelResponse**](RetrieveChannelResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

