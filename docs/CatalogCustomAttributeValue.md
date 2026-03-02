# CatalogCustomAttributeValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the custom attribute. | [optional]
**string_value** | Option<**String**> | The string value of the custom attribute.  Populated if `type` = `STRING`. | [optional]
**custom_attribute_definition_id** | Option<**String**> | The id of the [CatalogCustomAttributeDefinition](entity:CatalogCustomAttributeDefinition) this value belongs to. | [optional][readonly]
**r#type** | Option<[**models::CatalogCustomAttributeDefinitionType**](CatalogCustomAttributeDefinitionType.md)> |  | [optional]
**number_value** | Option<**String**> | Populated if `type` = `NUMBER`. Contains a string representation of a decimal number, using a `.` as the decimal separator. | [optional]
**boolean_value** | Option<**bool**> | A `true` or `false` value. Populated if `type` = `BOOLEAN`. | [optional]
**selection_uid_values** | Option<**Vec<String>**> | One or more choices from `allowed_selections`. Populated if `type` = `SELECTION`. | [optional]
**key** | Option<**String**> | If the associated `CatalogCustomAttributeDefinition` object is defined by another application, this key is prefixed by the defining application ID. For example, if the CatalogCustomAttributeDefinition has a key attribute of \"cocoa_brand\" and the defining application ID is \"abcd1234\", this key is \"abcd1234:cocoa_brand\" when the application making the request is different from the application defining the custom attribute definition. Otherwise, the key is simply \"cocoa_brand\". | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


