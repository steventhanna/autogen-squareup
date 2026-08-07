# InventoryAdjustmentReason

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**models::InventoryAdjustmentReasonId**](InventoryAdjustmentReasonId.md) |  | 
**name** | Option<**String**> | The seller-facing name for a custom inventory adjustment reason. This field is empty for standard and system-generated adjustment reasons. | [optional]
**direction** | Option<[**models::InventoryAdjustmentReasonDirection**](InventoryAdjustmentReasonDirection.md)> |  | [optional]
**created_at** | Option<**String**> | An RFC 3339-formatted timestamp that indicates when the custom adjustment reason was created. This field is empty for standard adjustment reasons. | [optional][readonly]
**updated_at** | Option<**String**> | An RFC 3339-formatted timestamp that indicates when the custom adjustment reason was last updated. This field is empty for standard adjustment reasons. | [optional][readonly]
**is_deleted** | Option<**bool**> | Indicates whether this custom inventory adjustment reason has been deleted. Deleted custom reasons can still be retrieved by ID, but are omitted from list responses unless deleted reasons are explicitly included. To restore a deleted custom reason, call [RestoreInventoryAdjustmentReason](api-endpoint:Inventory-RestoreInventoryAdjustmentReason). This field is always `false` for standard and system-generated adjustment reasons. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


