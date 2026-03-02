# UpdateTransferOrderLineData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uid** | Option<**String**> | Line item id being updated. Required for updating/removing existing line items, but should not be set for new line items. | [optional]
**item_variation_id** | Option<**String**> | Catalog item variation being transferred  Required for new line items, but otherwise is not updatable. | [optional]
**quantity_ordered** | Option<**String**> | Total quantity ordered | [optional]
**remove** | Option<**bool**> | Flag to remove the line item during update. Must include `uid` in removal request | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


