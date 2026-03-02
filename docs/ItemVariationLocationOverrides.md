# ItemVariationLocationOverrides

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_id** | Option<**String**> | The ID of the `Location`. This can include locations that are deactivated. | [optional]
**price_money** | Option<[**models::Money**](Money.md)> |  | [optional]
**pricing_type** | Option<[**models::CatalogPricingType**](CatalogPricingType.md)> |  | [optional]
**track_inventory** | Option<**bool**> | If `true`, inventory tracking is active for the `CatalogItemVariation` at this `Location`. | [optional]
**inventory_alert_type** | Option<[**models::InventoryAlertType**](InventoryAlertType.md)> |  | [optional]
**inventory_alert_threshold** | Option<**i64**> | If the inventory quantity for the variation is less than or equal to this value and `inventory_alert_type` is `LOW_QUANTITY`, the variation displays an alert in the merchant dashboard.  This value is always an integer. | [optional]
**sold_out** | Option<**bool**> | Indicates whether the overridden item variation is sold out at the specified location.  When inventory tracking is enabled on the item variation either globally or at the specified location, the item variation is automatically marked as sold out when its inventory count reaches zero. The seller can manually set the item variation as sold out even when the inventory count is greater than zero. Attempts by an application to set this attribute are ignored. Regardless how the sold-out status is set, applications should treat its inventory count as zero when this attribute value is `true`. | [optional][readonly]
**sold_out_valid_until** | Option<**String**> | The seller-assigned timestamp, of the RFC 3339 format, to indicate when this sold-out variation becomes available again at the specified location. Attempts by an application to set this attribute are ignored. When the current time is later than this attribute value, the affected item variation is no longer sold out. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


