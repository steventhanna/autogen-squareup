# OrderLineItemPricingBlocklistsBlockedServiceCharge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | A unique ID of the `BlockedServiceCharge` within the order. | [optional]
**service_charge_uid** | Option<**String**> | The `uid` of the service charge that should be blocked. Use this field to block ad hoc service charges. For catalog service charges, use the  `service_charge_catalog_object_id` field. | [optional]
**service_charge_catalog_object_id** | Option<**String**> | The `catalog_object_id` of the service charge that should be blocked. Use this field to block catalog service charges. For ad hoc service charges, use the `service_charge_uid` field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


