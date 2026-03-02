# CreateTransferOrderData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_location_id** | **String** | The source [Location](entity:Location) that will send the items. Must be an active location in your Square account with sufficient inventory of the requested items. | 
**destination_location_id** | **String** | The destination [Location](entity:Location) that will receive the items. Must be an active location in your Square account | 
**expected_at** | Option<**String**> | Expected transfer date in RFC 3339 format (e.g. \"2023-10-01T12:00:00Z\"). | [optional]
**notes** | Option<**String**> | Optional notes about the transfer | [optional]
**tracking_number** | Option<**String**> | Optional shipment tracking number | [optional]
**created_by_team_member_id** | Option<**String**> | ID of the [TeamMember](entity:TeamMember) creating this transfer order. Used for tracking and auditing purposes. | [optional]
**line_items** | Option<[**Vec<models::CreateTransferOrderLineData>**](CreateTransferOrderLineData.md)> | List of [CatalogItemVariation](entity:CatalogItemVariation)s to transfer, including quantities | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


