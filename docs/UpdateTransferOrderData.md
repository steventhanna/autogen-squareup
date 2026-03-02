# UpdateTransferOrderData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_location_id** | Option<**String**> | The source [Location](entity:Location) that will send the items. Must be an active location in your Square account with sufficient inventory of the requested items. | [optional]
**destination_location_id** | Option<**String**> | The destination [Location](entity:Location) that will receive the items. Must be an active location in your Square account. | [optional]
**expected_at** | Option<**String**> | Expected transfer date in RFC 3339 format (e.g. \"2023-10-01T12:00:00Z\"). | [optional]
**notes** | Option<**String**> | Optional notes about the transfer | [optional]
**tracking_number** | Option<**String**> | Shipment tracking number | [optional]
**line_items** | Option<[**Vec<models::UpdateTransferOrderLineData>**](UpdateTransferOrderLineData.md)> | List of items being transferred | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


