# CatalogObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::CatalogObjectType**](CatalogObjectType.md) |  | 
**id** | **String** | An identifier to reference this object in the catalog. When a new `CatalogObject` is inserted, the client should set the id to a temporary identifier starting with a \"`#`\" character. Other objects being inserted or updated within the same request may use this identifier to refer to the new object.  When the server receives the new object, it will supply a unique identifier that replaces the temporary identifier for all future references. | 
**updated_at** | Option<**String**> | Last modification [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) in RFC 3339 format, e.g., `\"2016-08-15T23:59:33.123Z\"` would indicate the UTC time (denoted by `Z`) of August 15, 2016 at 23:59:33 and 123 milliseconds. | [optional][readonly]
**version** | Option<**i64**> | The version of the object. When updating an object, the version supplied must match the version in the database, otherwise the write will be rejected as conflicting. | [optional]
**is_deleted** | Option<**bool**> | If `true`, the object has been deleted from the database. Must be `false` for new objects being inserted. When deleted, the `updated_at` field will equal the deletion time. | [optional]
**custom_attribute_values** | Option<[**std::collections::HashMap<String, models::CatalogCustomAttributeValue>**](CatalogCustomAttributeValue.md)> | A map (key-value pairs) of application-defined custom attribute values. The value of a key-value pair is a [CatalogCustomAttributeValue](entity:CatalogCustomAttributeValue) object. The key is the `key` attribute value defined in the associated [CatalogCustomAttributeDefinition](entity:CatalogCustomAttributeDefinition) object defined by the application making the request.  If the `CatalogCustomAttributeDefinition` object is defined by another application, the `CatalogCustomAttributeDefinition`'s key attribute value is prefixed by the defining application ID. For example, if the `CatalogCustomAttributeDefinition` has a `key` attribute of `\"cocoa_brand\"` and the defining application ID is `\"abcd1234\"`, the key in the map is `\"abcd1234:cocoa_brand\"` if the application making the request is different from the application defining the custom attribute definition. Otherwise, the key used in the map is simply `\"cocoa_brand\"`.  Application-defined custom attributes are set at a global (location-independent) level. Custom attribute values are intended to store additional information about a catalog object or associations with an entity in another system. Do not use custom attributes to store any sensitive information (personally identifiable information, card details, etc.). | [optional]
**catalog_v1_ids** | Option<[**Vec<models::CatalogV1Id>**](CatalogV1Id.md)> | The Connect v1 IDs for this object at each location where it is present, where they differ from the object's Connect V2 ID. The field will only be present for objects that have been created or modified by legacy APIs. | [optional]
**present_at_all_locations** | Option<**bool**> | If `true`, this object is present at all locations (including future locations), except where specified in the `absent_at_location_ids` field. If `false`, this object is not present at any locations (including future locations), except where specified in the `present_at_location_ids` field. If not specified, defaults to `true`. | [optional]
**present_at_location_ids** | Option<**Vec<String>**> | A list of locations where the object is present, even if `present_at_all_locations` is `false`. This can include locations that are deactivated. | [optional]
**absent_at_location_ids** | Option<**Vec<String>**> | A list of locations where the object is not present, even if `present_at_all_locations` is `true`. This can include locations that are deactivated. | [optional]
**item_data** | Option<[**models::CatalogItem**](CatalogItem.md)> |  | [optional]
**category_data** | Option<[**models::CatalogCategory**](CatalogCategory.md)> |  | [optional]
**item_variation_data** | Option<[**models::CatalogItemVariation**](CatalogItemVariation.md)> |  | [optional]
**tax_data** | Option<[**models::CatalogTax**](CatalogTax.md)> |  | [optional]
**discount_data** | Option<[**models::CatalogDiscount**](CatalogDiscount.md)> |  | [optional]
**modifier_list_data** | Option<[**models::CatalogModifierList**](CatalogModifierList.md)> |  | [optional]
**modifier_data** | Option<[**models::CatalogModifier**](CatalogModifier.md)> |  | [optional]
**time_period_data** | Option<[**models::CatalogTimePeriod**](CatalogTimePeriod.md)> |  | [optional]
**product_set_data** | Option<[**models::CatalogProductSet**](CatalogProductSet.md)> |  | [optional]
**pricing_rule_data** | Option<[**models::CatalogPricingRule**](CatalogPricingRule.md)> |  | [optional]
**image_data** | Option<[**models::CatalogImage**](CatalogImage.md)> |  | [optional]
**measurement_unit_data** | Option<[**models::CatalogMeasurementUnit**](CatalogMeasurementUnit.md)> |  | [optional]
**subscription_plan_data** | Option<[**models::CatalogSubscriptionPlan**](CatalogSubscriptionPlan.md)> |  | [optional]
**item_option_data** | Option<[**models::CatalogItemOption**](CatalogItemOption.md)> |  | [optional]
**item_option_value_data** | Option<[**models::CatalogItemOptionValue**](CatalogItemOptionValue.md)> |  | [optional]
**custom_attribute_definition_data** | Option<[**models::CatalogCustomAttributeDefinition**](CatalogCustomAttributeDefinition.md)> |  | [optional]
**quick_amounts_settings_data** | Option<[**models::CatalogQuickAmountsSettings**](CatalogQuickAmountsSettings.md)> |  | [optional]
**subscription_plan_variation_data** | Option<[**models::CatalogSubscriptionPlanVariation**](CatalogSubscriptionPlanVariation.md)> |  | [optional]
**availability_period_data** | Option<[**models::CatalogAvailabilityPeriod**](CatalogAvailabilityPeriod.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


