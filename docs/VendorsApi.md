# \VendorsApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_vendors**](VendorsApi.md#bulk_create_vendors) | **POST** /v2/vendors/bulk-create | BulkCreateVendors
[**bulk_retrieve_vendors**](VendorsApi.md#bulk_retrieve_vendors) | **POST** /v2/vendors/bulk-retrieve | BulkRetrieveVendors
[**bulk_update_vendors**](VendorsApi.md#bulk_update_vendors) | **PUT** /v2/vendors/bulk-update | BulkUpdateVendors
[**create_vendor**](VendorsApi.md#create_vendor) | **POST** /v2/vendors/create | CreateVendor
[**retrieve_vendor**](VendorsApi.md#retrieve_vendor) | **GET** /v2/vendors/{vendor_id} | RetrieveVendor
[**search_vendors**](VendorsApi.md#search_vendors) | **POST** /v2/vendors/search | SearchVendors
[**update_vendor**](VendorsApi.md#update_vendor) | **PUT** /v2/vendors/{vendor_id} | UpdateVendor



## bulk_create_vendors

> models::BulkCreateVendorsResponse bulk_create_vendors(bulk_create_vendors_request)
BulkCreateVendors

Creates one or more [Vendor](entity:Vendor) objects to represent suppliers to a seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_create_vendors_request** | [**BulkCreateVendorsRequest**](BulkCreateVendorsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkCreateVendorsResponse**](BulkCreateVendorsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_retrieve_vendors

> models::BulkRetrieveVendorsResponse bulk_retrieve_vendors(bulk_retrieve_vendors_request)
BulkRetrieveVendors

Retrieves one or more vendors of specified [Vendor](entity:Vendor) IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_retrieve_vendors_request** | [**BulkRetrieveVendorsRequest**](BulkRetrieveVendorsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkRetrieveVendorsResponse**](BulkRetrieveVendorsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_vendors

> models::BulkUpdateVendorsResponse bulk_update_vendors(bulk_update_vendors_request)
BulkUpdateVendors

Updates one or more of existing [Vendor](entity:Vendor) objects as suppliers to a seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_update_vendors_request** | [**BulkUpdateVendorsRequest**](BulkUpdateVendorsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpdateVendorsResponse**](BulkUpdateVendorsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vendor

> models::CreateVendorResponse create_vendor(create_vendor_request)
CreateVendor

Creates a single [Vendor](entity:Vendor) object to represent a supplier to a seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vendor_request** | [**CreateVendorRequest**](CreateVendorRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateVendorResponse**](CreateVendorResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_vendor

> models::RetrieveVendorResponse retrieve_vendor(vendor_id)
RetrieveVendor

Retrieves the vendor of a specified [Vendor](entity:Vendor) ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vendor_id** | **String** | ID of the [Vendor](entity:Vendor) to retrieve. | [required] |

### Return type

[**models::RetrieveVendorResponse**](RetrieveVendorResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_vendors

> models::SearchVendorsResponse search_vendors(search_vendors_request)
SearchVendors

Searches for vendors using a filter against supported [Vendor](entity:Vendor) properties and a supported sorter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_vendors_request** | [**SearchVendorsRequest**](SearchVendorsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchVendorsResponse**](SearchVendorsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vendor

> models::UpdateVendorResponse update_vendor(update_vendor_request)
UpdateVendor

Updates an existing [Vendor](entity:Vendor) object as a supplier to a seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_vendor_request** | [**UpdateVendorRequest**](UpdateVendorRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateVendorResponse**](UpdateVendorResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

