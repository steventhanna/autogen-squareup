# TransferOrderGoodsReceipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line_items** | Option<[**Vec<models::TransferOrderGoodsReceiptLineItem>**](TransferOrderGoodsReceiptLineItem.md)> | Line items being received. Each line item specifies: - The item being received - Quantity received in good condition - Quantity received damaged - Quantity canceled  Constraints: - Must include at least one line item - Maximum of 1000 line items per receipt - Each line item must reference a valid item from the transfer order - Total of received, damaged, and canceled quantities cannot exceed ordered quantity | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


