# \CatalogApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_catalog_objects**](CatalogApi.md#batch_delete_catalog_objects) | **POST** /v2/catalog/batch-delete | BatchDeleteCatalogObjects
[**batch_retrieve_catalog_objects**](CatalogApi.md#batch_retrieve_catalog_objects) | **POST** /v2/catalog/batch-retrieve | BatchRetrieveCatalogObjects
[**batch_upsert_catalog_objects**](CatalogApi.md#batch_upsert_catalog_objects) | **POST** /v2/catalog/batch-upsert | BatchUpsertCatalogObjects
[**catalog_info**](CatalogApi.md#catalog_info) | **GET** /v2/catalog/info | CatalogInfo
[**create_catalog_image**](CatalogApi.md#create_catalog_image) | **POST** /v2/catalog/images | CreateCatalogImage
[**delete_catalog_object**](CatalogApi.md#delete_catalog_object) | **DELETE** /v2/catalog/object/{object_id} | DeleteCatalogObject
[**list_catalog**](CatalogApi.md#list_catalog) | **GET** /v2/catalog/list | ListCatalog
[**retrieve_catalog_object**](CatalogApi.md#retrieve_catalog_object) | **GET** /v2/catalog/object/{object_id} | RetrieveCatalogObject
[**search_catalog_items**](CatalogApi.md#search_catalog_items) | **POST** /v2/catalog/search-catalog-items | SearchCatalogItems
[**search_catalog_objects**](CatalogApi.md#search_catalog_objects) | **POST** /v2/catalog/search | SearchCatalogObjects
[**update_catalog_image**](CatalogApi.md#update_catalog_image) | **PUT** /v2/catalog/images/{image_id} | UpdateCatalogImage
[**update_item_modifier_lists**](CatalogApi.md#update_item_modifier_lists) | **POST** /v2/catalog/update-item-modifier-lists | UpdateItemModifierLists
[**update_item_taxes**](CatalogApi.md#update_item_taxes) | **POST** /v2/catalog/update-item-taxes | UpdateItemTaxes
[**upsert_catalog_object**](CatalogApi.md#upsert_catalog_object) | **POST** /v2/catalog/object | UpsertCatalogObject



## batch_delete_catalog_objects

> models::BatchDeleteCatalogObjectsResponse batch_delete_catalog_objects(batch_delete_catalog_objects_request)
BatchDeleteCatalogObjects

Deletes a set of [CatalogItem](entity:CatalogItem)s based on the provided list of target IDs and returns a set of successfully deleted IDs in the response. Deletion is a cascading event such that all children of the targeted object are also deleted. For example, deleting a CatalogItem will also delete all of its [CatalogItemVariation](entity:CatalogItemVariation) children.  `BatchDeleteCatalogObjects` succeeds even if only a portion of the targeted IDs can be deleted. The response will only include IDs that were actually deleted.  To ensure consistency, only one delete request is processed at a time per seller account. While one (batch or non-batch) delete request is being processed, other (batched and non-batched) delete requests are rejected with the `429` error code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_catalog_objects_request** | [**BatchDeleteCatalogObjectsRequest**](BatchDeleteCatalogObjectsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchDeleteCatalogObjectsResponse**](BatchDeleteCatalogObjectsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_retrieve_catalog_objects

> models::BatchRetrieveCatalogObjectsResponse batch_retrieve_catalog_objects(batch_retrieve_catalog_objects_request)
BatchRetrieveCatalogObjects

Returns a set of objects based on the provided ID. Each [CatalogItem](entity:CatalogItem) returned in the set includes all of its child information including: all of its [CatalogItemVariation](entity:CatalogItemVariation) objects, references to its [CatalogModifierList](entity:CatalogModifierList) objects, and the ids of any [CatalogTax](entity:CatalogTax) objects that apply to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_retrieve_catalog_objects_request** | [**BatchRetrieveCatalogObjectsRequest**](BatchRetrieveCatalogObjectsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchRetrieveCatalogObjectsResponse**](BatchRetrieveCatalogObjectsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_upsert_catalog_objects

> models::BatchUpsertCatalogObjectsResponse batch_upsert_catalog_objects(batch_upsert_catalog_objects_request)
BatchUpsertCatalogObjects

Creates or updates up to 10,000 target objects based on the provided list of objects. The target objects are grouped into batches and each batch is inserted/updated in an all-or-nothing manner. If an object within a batch is malformed in some way, or violates a database constraint, the entire batch containing that item will be disregarded. However, other batches in the same request may still succeed. Each batch may contain up to 1,000 objects, and batches will be processed in order as long as the total object count for the request (items, variations, modifier lists, discounts, and taxes) is no more than 10,000.  To ensure consistency, only one update request is processed at a time per seller account. While one (batch or non-batch) update request is being processed, other (batched and non-batched) update requests are rejected with the `429` error code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_upsert_catalog_objects_request** | [**BatchUpsertCatalogObjectsRequest**](BatchUpsertCatalogObjectsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BatchUpsertCatalogObjectsResponse**](BatchUpsertCatalogObjectsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## catalog_info

> models::CatalogInfoResponse catalog_info()
CatalogInfo

Retrieves information about the Square Catalog API, such as batch size limits that can be used by the `BatchUpsertCatalogObjects` endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CatalogInfoResponse**](CatalogInfoResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_catalog_image

> models::CreateCatalogImageResponse create_catalog_image(request, image_file)
CreateCatalogImage

Uploads an image file to be represented by a [CatalogImage](entity:CatalogImage) object that can be linked to an existing [CatalogObject](entity:CatalogObject) instance. The resulting `CatalogImage` is unattached to any `CatalogObject` if the `object_id` is not specified.  This `CreateCatalogImage` endpoint accepts HTTP multipart/form-data requests with a JSON part and an image file part in JPEG, PJPEG, PNG, or GIF format. The maximum file size is 15MB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | Option<[**models::CreateCatalogImageRequest**](CreateCatalogImageRequest.md)> |  |  |
**image_file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::CreateCatalogImageResponse**](CreateCatalogImageResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_catalog_object

> models::DeleteCatalogObjectResponse delete_catalog_object(object_id)
DeleteCatalogObject

Deletes a single [CatalogObject](entity:CatalogObject) based on the provided ID and returns the set of successfully deleted IDs in the response. Deletion is a cascading event such that all children of the targeted object are also deleted. For example, deleting a [CatalogItem](entity:CatalogItem) will also delete all of its [CatalogItemVariation](entity:CatalogItemVariation) children.  To ensure consistency, only one delete request is processed at a time per seller account. While one (batch or non-batch) delete request is being processed, other (batched and non-batched) delete requests are rejected with the `429` error code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | The ID of the catalog object to be deleted. When an object is deleted, other objects in the graph that depend on that object will be deleted as well (for example, deleting a catalog item will delete its catalog item variations). | [required] |

### Return type

[**models::DeleteCatalogObjectResponse**](DeleteCatalogObjectResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_catalog

> models::ListCatalogResponse list_catalog(cursor, types, catalog_version)
ListCatalog

Returns a list of all [CatalogObject](entity:CatalogObject)s of the specified types in the catalog.  The `types` parameter is specified as a comma-separated list of the [CatalogObjectType](entity:CatalogObjectType) values, for example, \"`ITEM`, `ITEM_VARIATION`, `MODIFIER`, `MODIFIER_LIST`, `CATEGORY`, `DISCOUNT`, `TAX`, `IMAGE`\".  __Important:__ ListCatalog does not return deleted catalog items. To retrieve deleted catalog items, use [SearchCatalogObjects](api-endpoint:Catalog-SearchCatalogObjects) and set the `include_deleted_objects` attribute value to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor returned in the previous response. Leave unset for an initial request. The page size is currently set to be 100. See [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination) for more information. |  |
**types** | Option<**String**> | An optional case-insensitive, comma-separated list of object types to retrieve.  The valid values are defined in the [CatalogObjectType](entity:CatalogObjectType) enum, for example, `ITEM`, `ITEM_VARIATION`, `CATEGORY`, `DISCOUNT`, `TAX`, `MODIFIER`, `MODIFIER_LIST`, `IMAGE`, etc.  If this is unspecified, the operation returns objects of all the top level types at the version of the Square API used to make the request. Object types that are nested onto other object types are not included in the defaults.  At the current API version the default object types are: ITEM, CATEGORY, TAX, DISCOUNT, MODIFIER_LIST,  PRICING_RULE, PRODUCT_SET, TIME_PERIOD, MEASUREMENT_UNIT, SUBSCRIPTION_PLAN, ITEM_OPTION, CUSTOM_ATTRIBUTE_DEFINITION, QUICK_AMOUNT_SETTINGS. |  |
**catalog_version** | Option<**i64**> | The specific version of the catalog objects to be included in the response. This allows you to retrieve historical versions of objects. The specified version value is matched against the [CatalogObject](entity:CatalogObject)s' `version` attribute.  If not included, results will be from the current version of the catalog. |  |

### Return type

[**models::ListCatalogResponse**](ListCatalogResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_catalog_object

> models::RetrieveCatalogObjectResponse retrieve_catalog_object(object_id, include_related_objects, catalog_version, include_category_path_to_root)
RetrieveCatalogObject

Returns a single [CatalogItem](entity:CatalogItem) as a [CatalogObject](entity:CatalogObject) based on the provided ID. The returned object includes all of the relevant [CatalogItem](entity:CatalogItem) information including: [CatalogItemVariation](entity:CatalogItemVariation) children, references to its [CatalogModifierList](entity:CatalogModifierList) objects, and the ids of any [CatalogTax](entity:CatalogTax) objects that apply to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | The object ID of any type of catalog objects to be retrieved. | [required] |
**include_related_objects** | Option<**bool**> | If `true`, the response will include additional objects that are related to the requested objects. Related objects are defined as any objects referenced by ID by the results in the `objects` field of the response. These objects are put in the `related_objects` field. Setting this to `true` is helpful when the objects are needed for immediate display to a user. This process only goes one level deep. Objects referenced by the related objects will not be included. For example,  if the `objects` field of the response contains a CatalogItem, its associated CatalogCategory objects, CatalogTax objects, CatalogImage objects and CatalogModifierLists will be returned in the `related_objects` field of the response. If the `objects` field of the response contains a CatalogItemVariation, its parent CatalogItem will be returned in the `related_objects` field of the response.  Default value: `false` |  |[default to false]
**catalog_version** | Option<**i64**> | Requests objects as of a specific version of the catalog. This allows you to retrieve historical versions of objects. The value to retrieve a specific version of an object can be found in the version field of [CatalogObject](entity:CatalogObject)s. If not included, results will be from the current version of the catalog. |  |
**include_category_path_to_root** | Option<**bool**> | Specifies whether or not to include the `path_to_root` list for each returned category instance. The `path_to_root` list consists of `CategoryPathToRootNode` objects and specifies the path that starts with the immediate parent category of the returned category and ends with its root category. If the returned category is a top-level category, the `path_to_root` list is empty and is not returned in the response payload. |  |[default to false]

### Return type

[**models::RetrieveCatalogObjectResponse**](RetrieveCatalogObjectResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_catalog_items

> models::SearchCatalogItemsResponse search_catalog_items(search_catalog_items_request)
SearchCatalogItems

Searches for catalog items or item variations by matching supported search attribute values, including custom attribute values, against one or more of the specified query filters.  This (`SearchCatalogItems`) endpoint differs from the [SearchCatalogObjects](api-endpoint:Catalog-SearchCatalogObjects) endpoint in the following aspects:  - `SearchCatalogItems` can only search for items or item variations, whereas `SearchCatalogObjects` can search for any type of catalog objects. - `SearchCatalogItems` supports the custom attribute query filters to return items or item variations that contain custom attribute values, where `SearchCatalogObjects` does not. - `SearchCatalogItems` does not support the `include_deleted_objects` filter to search for deleted items or item variations, whereas `SearchCatalogObjects` does. - The both endpoints use different call conventions, including the query filter formats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_catalog_items_request** | [**SearchCatalogItemsRequest**](SearchCatalogItemsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchCatalogItemsResponse**](SearchCatalogItemsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_catalog_objects

> models::SearchCatalogObjectsResponse search_catalog_objects(search_catalog_objects_request)
SearchCatalogObjects

Searches for [CatalogObject](entity:CatalogObject) of any type by matching supported search attribute values, excluding custom attribute values on items or item variations, against one or more of the specified query filters.  This (`SearchCatalogObjects`) endpoint differs from the [SearchCatalogItems](api-endpoint:Catalog-SearchCatalogItems) endpoint in the following aspects:  - `SearchCatalogItems` can only search for items or item variations, whereas `SearchCatalogObjects` can search for any type of catalog objects. - `SearchCatalogItems` supports the custom attribute query filters to return items or item variations that contain custom attribute values, where `SearchCatalogObjects` does not. - `SearchCatalogItems` does not support the `include_deleted_objects` filter to search for deleted items or item variations, whereas `SearchCatalogObjects` does. - The both endpoints have different call conventions, including the query filter formats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_catalog_objects_request** | [**SearchCatalogObjectsRequest**](SearchCatalogObjectsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchCatalogObjectsResponse**](SearchCatalogObjectsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_catalog_image

> models::UpdateCatalogImageResponse update_catalog_image(image_id, request, image_file)
UpdateCatalogImage

Uploads a new image file to replace the existing one in the specified [CatalogImage](entity:CatalogImage) object.  This `UpdateCatalogImage` endpoint accepts HTTP multipart/form-data requests with a JSON part and an image file part in JPEG, PJPEG, PNG, or GIF format. The maximum file size is 15MB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** | The ID of the `CatalogImage` object to update the encapsulated image file. | [required] |
**request** | Option<[**models::UpdateCatalogImageRequest**](UpdateCatalogImageRequest.md)> |  |  |
**image_file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::UpdateCatalogImageResponse**](UpdateCatalogImageResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_modifier_lists

> models::UpdateItemModifierListsResponse update_item_modifier_lists(update_item_modifier_lists_request)
UpdateItemModifierLists

Updates the [CatalogModifierList](entity:CatalogModifierList) objects that apply to the targeted [CatalogItem](entity:CatalogItem) without having to perform an upsert on the entire item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_item_modifier_lists_request** | [**UpdateItemModifierListsRequest**](UpdateItemModifierListsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateItemModifierListsResponse**](UpdateItemModifierListsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_taxes

> models::UpdateItemTaxesResponse update_item_taxes(update_item_taxes_request)
UpdateItemTaxes

Updates the [CatalogTax](entity:CatalogTax) objects that apply to the targeted [CatalogItem](entity:CatalogItem) without having to perform an upsert on the entire item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_item_taxes_request** | [**UpdateItemTaxesRequest**](UpdateItemTaxesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateItemTaxesResponse**](UpdateItemTaxesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_catalog_object

> models::UpsertCatalogObjectResponse upsert_catalog_object(upsert_catalog_object_request)
UpsertCatalogObject

Creates a new or updates the specified [CatalogObject](entity:CatalogObject).  To ensure consistency, only one update request is processed at a time per seller account. While one (batch or non-batch) update request is being processed, other (batched and non-batched) update requests are rejected with the `429` error code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upsert_catalog_object_request** | [**UpsertCatalogObjectRequest**](UpsertCatalogObjectRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpsertCatalogObjectResponse**](UpsertCatalogObjectResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

