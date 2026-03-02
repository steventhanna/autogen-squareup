# TransferOrderGoodsReceiptLineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_order_line_uid** | **String** | The unique identifier of the Transfer Order line being received | 
**quantity_received** | Option<**String**> | The quantity received for this line item as a decimal string (e.g. \"10.5\"). These items will be added to the destination [Location](entity:Location)'s inventory with [InventoryState](entity:InventoryState) of IN_STOCK. | [optional]
**quantity_damaged** | Option<**String**> | The quantity that was damaged during shipping/handling as a decimal string (e.g. \"1.5\"). These items will be added to the destination [Location](entity:Location)'s inventory with [InventoryState](entity:InventoryState) of WASTE. | [optional]
**quantity_canceled** | Option<**String**> | The quantity that was canceled during shipping/handling as a decimal string (e.g. \"1.5\"). These will be immediately added to inventory in the source location. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


