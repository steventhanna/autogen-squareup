# OrderCreated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | Option<**String**> | The order's unique ID. | [optional]
**version** | Option<**i32**> | The version number, which is incremented each time an update is committed to the order. Orders that were not created through the API do not include a version number and therefore cannot be updated.  [Read more about working with versions.](https://developer.squareup.com/docs/orders-api/manage-orders/update-orders) | [optional]
**location_id** | Option<**String**> | The ID of the seller location that this order is associated with. | [optional]
**state** | Option<[**models::OrderState**](OrderState.md)> |  | [optional]
**created_at** | Option<**String**> | The timestamp for when the order was created, in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


