# \InventoryApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_change_inventory**](InventoryApi.md#batch_change_inventory) | **POST** /v2/inventory/changes/batch-create | BatchChangeInventory
[**batch_retrieve_inventory_changes**](InventoryApi.md#batch_retrieve_inventory_changes) | **POST** /v2/inventory/changes/batch-retrieve | BatchRetrieveInventoryChanges
[**batch_retrieve_inventory_counts**](InventoryApi.md#batch_retrieve_inventory_counts) | **POST** /v2/inventory/counts/batch-retrieve | BatchRetrieveInventoryCounts
[**deprecated_batch_change_inventory**](InventoryApi.md#deprecated_batch_change_inventory) | **POST** /v2/inventory/batch-change | DeprecatedBatchChangeInventory
[**deprecated_batch_retrieve_inventory_changes**](InventoryApi.md#deprecated_batch_retrieve_inventory_changes) | **POST** /v2/inventory/batch-retrieve-changes | DeprecatedBatchRetrieveInventoryChanges
[**deprecated_batch_retrieve_inventory_counts**](InventoryApi.md#deprecated_batch_retrieve_inventory_counts) | **POST** /v2/inventory/batch-retrieve-counts | DeprecatedBatchRetrieveInventoryCounts
[**deprecated_retrieve_inventory_adjustment**](InventoryApi.md#deprecated_retrieve_inventory_adjustment) | **GET** /v2/inventory/adjustment/{adjustment_id} | DeprecatedRetrieveInventoryAdjustment
[**deprecated_retrieve_inventory_physical_count**](InventoryApi.md#deprecated_retrieve_inventory_physical_count) | **GET** /v2/inventory/physical-count/{physical_count_id} | DeprecatedRetrieveInventoryPhysicalCount
[**retrieve_inventory_adjustment**](InventoryApi.md#retrieve_inventory_adjustment) | **GET** /v2/inventory/adjustments/{adjustment_id} | RetrieveInventoryAdjustment
[**retrieve_inventory_changes**](InventoryApi.md#retrieve_inventory_changes) | **GET** /v2/inventory/{catalog_object_id}/changes | RetrieveInventoryChanges
[**retrieve_inventory_count**](InventoryApi.md#retrieve_inventory_count) | **GET** /v2/inventory/{catalog_object_id} | RetrieveInventoryCount
[**retrieve_inventory_physical_count**](InventoryApi.md#retrieve_inventory_physical_count) | **GET** /v2/inventory/physical-counts/{physical_count_id} | RetrieveInventoryPhysicalCount
[**retrieve_inventory_transfer**](InventoryApi.md#retrieve_inventory_transfer) | **GET** /v2/inventory/transfers/{transfer_id} | RetrieveInventoryTransfer



## batch_change_inventory

> models::BatchChangeInventoryResponse batch_change_inventory(batch_change_inventory_request)
BatchChangeInventory

Applies adjustments and counts to the provided item quantities.  On success: returns the current calculated counts for all objects referenced in the request. On failure: returns a list of related errors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_change_inventory_request** | [**BatchChangeInventoryRequest**](BatchChangeInventoryRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchChangeInventoryResponse**](BatchChangeInventoryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_retrieve_inventory_changes

> models::BatchRetrieveInventoryChangesResponse batch_retrieve_inventory_changes(batch_retrieve_inventory_changes_request)
BatchRetrieveInventoryChanges

Returns historical physical counts and adjustments based on the provided filter criteria.  Results are paginated and sorted in ascending order according their `occurred_at` timestamp (oldest first).  BatchRetrieveInventoryChanges is a catch-all query endpoint for queries that cannot be handled by other, simpler endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_retrieve_inventory_changes_request** | [**BatchRetrieveInventoryChangesRequest**](BatchRetrieveInventoryChangesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchRetrieveInventoryChangesResponse**](BatchRetrieveInventoryChangesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_retrieve_inventory_counts

> models::BatchRetrieveInventoryCountsResponse batch_retrieve_inventory_counts(batch_retrieve_inventory_counts_request)
BatchRetrieveInventoryCounts

Returns current counts for the provided [CatalogObject](entity:CatalogObject)s at the requested [Location](entity:Location)s.  Results are paginated and sorted in descending order according to their `calculated_at` timestamp (newest first).  When `updated_after` is specified, only counts that have changed since that time (based on the server timestamp for the most recent change) are returned. This allows clients to perform a \"sync\" operation, for example in response to receiving a Webhook notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_retrieve_inventory_counts_request** | [**BatchRetrieveInventoryCountsRequest**](BatchRetrieveInventoryCountsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchRetrieveInventoryCountsResponse**](BatchRetrieveInventoryCountsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_batch_change_inventory

> models::BatchChangeInventoryResponse deprecated_batch_change_inventory(batch_change_inventory_request)
DeprecatedBatchChangeInventory

Deprecated version of [BatchChangeInventory](api-endpoint:Inventory-BatchChangeInventory) after the endpoint URL is updated to conform to the standard convention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_change_inventory_request** | [**BatchChangeInventoryRequest**](BatchChangeInventoryRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchChangeInventoryResponse**](BatchChangeInventoryResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_batch_retrieve_inventory_changes

> models::BatchRetrieveInventoryChangesResponse deprecated_batch_retrieve_inventory_changes(batch_retrieve_inventory_changes_request)
DeprecatedBatchRetrieveInventoryChanges

Deprecated version of [BatchRetrieveInventoryChanges](api-endpoint:Inventory-BatchRetrieveInventoryChanges) after the endpoint URL is updated to conform to the standard convention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_retrieve_inventory_changes_request** | [**BatchRetrieveInventoryChangesRequest**](BatchRetrieveInventoryChangesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchRetrieveInventoryChangesResponse**](BatchRetrieveInventoryChangesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_batch_retrieve_inventory_counts

> models::BatchRetrieveInventoryCountsResponse deprecated_batch_retrieve_inventory_counts(batch_retrieve_inventory_counts_request)
DeprecatedBatchRetrieveInventoryCounts

Deprecated version of [BatchRetrieveInventoryCounts](api-endpoint:Inventory-BatchRetrieveInventoryCounts) after the endpoint URL is updated to conform to the standard convention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_retrieve_inventory_counts_request** | [**BatchRetrieveInventoryCountsRequest**](BatchRetrieveInventoryCountsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchRetrieveInventoryCountsResponse**](BatchRetrieveInventoryCountsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_retrieve_inventory_adjustment

> models::RetrieveInventoryAdjustmentResponse deprecated_retrieve_inventory_adjustment(adjustment_id)
DeprecatedRetrieveInventoryAdjustment

Deprecated version of [RetrieveInventoryAdjustment](api-endpoint:Inventory-RetrieveInventoryAdjustment) after the endpoint URL is updated to conform to the standard convention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**adjustment_id** | **String** | ID of the [InventoryAdjustment](entity:InventoryAdjustment) to retrieve. | [required] |

### Return type

[**models::RetrieveInventoryAdjustmentResponse**](RetrieveInventoryAdjustmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_retrieve_inventory_physical_count

> models::RetrieveInventoryPhysicalCountResponse deprecated_retrieve_inventory_physical_count(physical_count_id)
DeprecatedRetrieveInventoryPhysicalCount

Deprecated version of [RetrieveInventoryPhysicalCount](api-endpoint:Inventory-RetrieveInventoryPhysicalCount) after the endpoint URL is updated to conform to the standard convention.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**physical_count_id** | **String** | ID of the [InventoryPhysicalCount](entity:InventoryPhysicalCount) to retrieve. | [required] |

### Return type

[**models::RetrieveInventoryPhysicalCountResponse**](RetrieveInventoryPhysicalCountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_inventory_adjustment

> models::RetrieveInventoryAdjustmentResponse retrieve_inventory_adjustment(adjustment_id)
RetrieveInventoryAdjustment

Returns the [InventoryAdjustment](entity:InventoryAdjustment) object with the provided `adjustment_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**adjustment_id** | **String** | ID of the [InventoryAdjustment](entity:InventoryAdjustment) to retrieve. | [required] |

### Return type

[**models::RetrieveInventoryAdjustmentResponse**](RetrieveInventoryAdjustmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_inventory_changes

> models::RetrieveInventoryChangesResponse retrieve_inventory_changes(catalog_object_id, location_ids, cursor)
RetrieveInventoryChanges

Returns a set of physical counts and inventory adjustments for the provided [CatalogObject](entity:CatalogObject) at the requested [Location](entity:Location)s.  You can achieve the same result by calling [BatchRetrieveInventoryChanges](api-endpoint:Inventory-BatchRetrieveInventoryChanges) and having the `catalog_object_ids` list contain a single element of the `CatalogObject` ID.  Results are paginated and sorted in descending order according to their `occurred_at` timestamp (newest first).  There are no limits on how far back the caller can page. This endpoint can be used to display recent changes for a specific item. For more sophisticated queries, use a batch endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_object_id** | **String** | ID of the [CatalogObject](entity:CatalogObject) to retrieve. | [required] |
**location_ids** | Option<**String**> | The [Location](entity:Location) IDs to look up as a comma-separated list. An empty list queries all locations. |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for the original query.  See the [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination) guide for more information. |  |

### Return type

[**models::RetrieveInventoryChangesResponse**](RetrieveInventoryChangesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_inventory_count

> models::RetrieveInventoryCountResponse retrieve_inventory_count(catalog_object_id, location_ids, cursor)
RetrieveInventoryCount

Retrieves the current calculated stock count for a given [CatalogObject](entity:CatalogObject) at a given set of [Location](entity:Location)s. Responses are paginated and unsorted. For more sophisticated queries, use a batch endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_object_id** | **String** | ID of the [CatalogObject](entity:CatalogObject) to retrieve. | [required] |
**location_ids** | Option<**String**> | The [Location](entity:Location) IDs to look up as a comma-separated list. An empty list queries all locations. |  |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for the original query.  See the [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination) guide for more information. |  |

### Return type

[**models::RetrieveInventoryCountResponse**](RetrieveInventoryCountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_inventory_physical_count

> models::RetrieveInventoryPhysicalCountResponse retrieve_inventory_physical_count(physical_count_id)
RetrieveInventoryPhysicalCount

Returns the [InventoryPhysicalCount](entity:InventoryPhysicalCount) object with the provided `physical_count_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**physical_count_id** | **String** | ID of the [InventoryPhysicalCount](entity:InventoryPhysicalCount) to retrieve. | [required] |

### Return type

[**models::RetrieveInventoryPhysicalCountResponse**](RetrieveInventoryPhysicalCountResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_inventory_transfer

> models::RetrieveInventoryTransferResponse retrieve_inventory_transfer(transfer_id)
RetrieveInventoryTransfer

Returns the [InventoryTransfer](entity:InventoryTransfer) object with the provided `transfer_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_id** | **String** | ID of the [InventoryTransfer](entity:InventoryTransfer) to retrieve. | [required] |

### Return type

[**models::RetrieveInventoryTransferResponse**](RetrieveInventoryTransferResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

