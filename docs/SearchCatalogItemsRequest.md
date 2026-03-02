# SearchCatalogItemsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text_filter** | Option<**String**> | The text filter expression to return items or item variations containing specified text in the `name`, `description`, or `abbreviation` attribute value of an item, or in the `name`, `sku`, or `upc` attribute value of an item variation. | [optional]
**category_ids** | Option<**Vec<String>**> | The category id query expression to return items containing the specified category IDs. | [optional]
**stock_levels** | Option<[**Vec<models::SearchCatalogItemsRequestStockLevel>**](SearchCatalogItemsRequestStockLevel.md)> | The stock-level query expression to return item variations with the specified stock levels. See [SearchCatalogItemsRequestStockLevel](#type-searchcatalogitemsrequeststocklevel) for possible values | [optional]
**enabled_location_ids** | Option<**Vec<String>**> | The enabled-location query expression to return items and item variations having specified enabled locations. | [optional]
**cursor** | Option<**String**> | The pagination token, returned in the previous response, used to fetch the next batch of pending results. | [optional]
**limit** | Option<**i32**> | The maximum number of results to return per page. The default value is 100. | [optional]
**sort_order** | Option<[**models::SortOrder**](SortOrder.md)> |  | [optional]
**product_types** | Option<[**Vec<models::CatalogItemProductType>**](CatalogItemProductType.md)> | The product types query expression to return items or item variations having the specified product types. | [optional]
**custom_attribute_filters** | Option<[**Vec<models::CustomAttributeFilter>**](CustomAttributeFilter.md)> | The customer-attribute filter to return items or item variations matching the specified custom attribute expressions. A maximum number of 10 custom attribute expressions are supported in a single call to the [SearchCatalogItems](api-endpoint:Catalog-SearchCatalogItems) endpoint. | [optional]
**archived_state** | Option<[**models::ArchivedState**](ArchivedState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


