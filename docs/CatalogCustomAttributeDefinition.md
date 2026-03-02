# CatalogCustomAttributeDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::CatalogCustomAttributeDefinitionType**](CatalogCustomAttributeDefinitionType.md) |  | 
**name** | **String** |  The name of this definition for API and seller-facing UI purposes. The name must be unique within the (merchant, application) pair. Required. May not be empty and may not exceed 255 characters. Can be modified after creation. | 
**description** | Option<**String**> | Seller-oriented description of the meaning of this Custom Attribute, any constraints that the seller should observe, etc. May be displayed as a tooltip in Square UIs. | [optional]
**source_application** | Option<[**models::SourceApplication**](SourceApplication.md)> |  | [optional]
**allowed_object_types** | [**Vec<models::CatalogObjectType>**](CatalogObjectType.md) | The set of `CatalogObject` types that this custom atttribute may be applied to. Currently, only `ITEM`, `ITEM_VARIATION`, `MODIFIER`, `MODIFIER_LIST`, and `CATEGORY` are allowed. At least one type must be included. See [CatalogObjectType](#type-catalogobjecttype) for possible values | 
**seller_visibility** | Option<[**models::CatalogCustomAttributeDefinitionSellerVisibility**](CatalogCustomAttributeDefinitionSellerVisibility.md)> |  | [optional]
**app_visibility** | Option<[**models::CatalogCustomAttributeDefinitionAppVisibility**](CatalogCustomAttributeDefinitionAppVisibility.md)> |  | [optional]
**string_config** | Option<[**models::CatalogCustomAttributeDefinitionStringConfig**](CatalogCustomAttributeDefinitionStringConfig.md)> |  | [optional]
**number_config** | Option<[**models::CatalogCustomAttributeDefinitionNumberConfig**](CatalogCustomAttributeDefinitionNumberConfig.md)> |  | [optional]
**selection_config** | Option<[**models::CatalogCustomAttributeDefinitionSelectionConfig**](CatalogCustomAttributeDefinitionSelectionConfig.md)> |  | [optional]
**custom_attribute_usage_count** | Option<**i32**> | The number of custom attributes that reference this custom attribute definition. Set by the server in response to a ListCatalog request with `include_counts` set to `true`.  If the actual count is greater than 100, `custom_attribute_usage_count` will be set to `100`. | [optional][readonly]
**key** | Option<**String**> | The name of the desired custom attribute key that can be used to access the custom attribute value on catalog objects. Cannot be modified after the custom attribute definition has been created. Must be between 1 and 60 characters, and may only contain the characters `[a-zA-Z0-9_-]`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


