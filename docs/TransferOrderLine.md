# TransferOrderLine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | Unique system-generated identifier for the line item. Provide when updating/removing a line via [UpdateTransferOrder](api-endpoint:TransferOrders-UpdateTransferOrder). | [optional][readonly]
**item_variation_id** | **String** | The required identifier of the [CatalogItemVariation](entity:CatalogItemVariation) being transferred. Must reference a valid catalog item variation that exists in the [Catalog](api:Catalog). | 
**quantity_ordered** | **String** | Total quantity ordered, formatted as a decimal string (e.g. \"10 or 10.0000\"). Required to be a positive number.  To remove a line item, set `remove` to `true` in [UpdateTransferOrder](api-endpoint:TransferOrders-UpdateTransferOrder). | 
**quantity_pending** | Option<**String**> | Calculated quantity of this line item's yet to be received stock. This is the difference between the total quantity ordered and the sum of quantities received, canceled, and damaged. | [optional][readonly]
**quantity_received** | Option<**String**> | Quantity received at destination. These items are added to the destination [Location](entity:Location)'s inventory with [InventoryState](entity:InventoryState) of IN_STOCK.  This field cannot be updated directly in Create/Update operations, instead use [ReceiveTransferOrder](api-endpoint:TransferOrders-ReceiveTransferOrder). | [optional][readonly]
**quantity_damaged** | Option<**String**> | Quantity received in damaged condition. These items are added to the destination [Location](entity:Location)'s inventory with [InventoryState](entity:InventoryState) of WASTE.  This field cannot be updated directly in Create/Update operations, instead use [ReceiveTransferOrder](api-endpoint:TransferOrders-ReceiveTransferOrder). | [optional][readonly]
**quantity_canceled** | Option<**String**> | Quantity that was canceled. These items will be immediately added to inventory in the source location.  This field cannot be updated directly in Create/Update operations, instead use [ReceiveTransferOrder](api-endpoint:TransferOrders-ReceiveTransferOrder) or [CancelTransferOrder](api-endpoint:TransferOrders-CancelTransferOrder). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


