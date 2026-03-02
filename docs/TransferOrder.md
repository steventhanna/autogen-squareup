# TransferOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique system-generated identifier for this transfer order. Use this ID for: - Retrieving transfer order details - Tracking status changes via webhooks - Linking transfers in external systems | [optional][readonly]
**source_location_id** | Option<**String**> | The source [Location](entity:Location) sending the [CatalogItemVariation](entity:CatalogItemVariation)s. This location must: - Be active in your Square organization - Have sufficient inventory for the items being transferred - Not be the same as the destination location  This field is not updatable. | [optional]
**destination_location_id** | Option<**String**> | The destination [Location](entity:Location) receiving the [CatalogItemVariation](entity:CatalogItemVariation)s. This location must: - Be active in your Square organization - Not be the same as the source location  This field is not updatable. | [optional]
**status** | Option<[**models::TransferOrderStatus**](TransferOrderStatus.md)> |  | [optional]
**created_at** | Option<**String**> | Timestamp when the transfer order was created, in RFC 3339 format. Used for: - Auditing transfer history - Tracking order age - Reporting and analytics | [optional][readonly]
**updated_at** | Option<**String**> | Timestamp when the transfer order was last updated, in RFC 3339 format. Updated when: - Order status changes - Items are received - Notes or metadata are modified | [optional][readonly]
**expected_at** | Option<**String**> | Expected transfer completion date, in RFC 3339 format. Used for: - Planning inventory availability - Scheduling receiving staff - Monitoring transfer timeliness | [optional]
**completed_at** | Option<**String**> | Timestamp when the transfer order was completed or canceled, in RFC 3339 format (e.g. \"2023-10-01T12:00:00Z\"). | [optional][readonly]
**notes** | Option<**String**> | Optional notes about the transfer. | [optional]
**tracking_number** | Option<**String**> | Shipment tracking number for monitoring transfer progress. | [optional]
**created_by_team_member_id** | Option<**String**> | ID of the [TeamMember](entity:TeamMember) who created this transfer order. This field is not writeable by the Connect V2 API. | [optional][readonly]
**line_items** | Option<[**Vec<models::TransferOrderLine>**](TransferOrderLine.md)> | List of [CatalogItemVariation](entity:CatalogItemVariation)s being transferred. | [optional]
**version** | Option<**i64**> | Version for optimistic concurrency control. This is a monotonically increasing integer that changes whenever the transfer order is modified. Use this when calling  [UpdateTransferOrder](api-endpoint:TransferOrders-UpdateTransferOrder) and other endpoints to ensure you're not overwriting concurrent changes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


