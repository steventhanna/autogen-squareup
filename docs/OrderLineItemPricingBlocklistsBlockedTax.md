# OrderLineItemPricingBlocklistsBlockedTax

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID of the `BlockedTax` within the order. | [optional]
**tax_uid** | Option<**String**> | The `uid` of the tax that should be blocked. Use this field to block ad hoc taxes. For catalog, taxes use the `tax_catalog_object_id` field. | [optional]
**tax_catalog_object_id** | Option<**String**> | The `catalog_object_id` of the tax that should be blocked. Use this field to block catalog taxes. For ad hoc taxes, use the `tax_uid` field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


