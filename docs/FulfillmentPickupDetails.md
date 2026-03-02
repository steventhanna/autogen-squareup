# FulfillmentPickupDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipient** | Option<[**models::FulfillmentRecipient**](FulfillmentRecipient.md)> |  | [optional]
**expires_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment expires if it is not marked in progress. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). The expiration time can only be set up to 7 days in the future. If `expires_at` is not set, any new payments attached to the order are automatically completed. | [optional]
**auto_complete_duration** | Option<**String**> | The duration of time after which an in progress pickup fulfillment is automatically moved to the `COMPLETED` state. The duration must be in RFC 3339 format (for example, \"P1W3D\").  If not set, this pickup fulfillment remains in progress until it is canceled or completed. | [optional]
**schedule_type** | Option<[**models::FulfillmentPickupDetailsScheduleType**](FulfillmentPickupDetailsScheduleType.md)> |  | [optional]
**pickup_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) that represents the start of the pickup window. Must be in RFC 3339 timestamp format, e.g., \"2016-09-04T23:59:33.123Z\".  For fulfillments with the schedule type `ASAP`, this is automatically set to the current time plus the expected duration to prepare the fulfillment. | [optional]
**pickup_window_duration** | Option<**String**> | The window of time in which the order should be picked up after the `pickup_at` timestamp. Must be in RFC 3339 duration format, e.g., \"P1W3D\". Can be used as an informational guideline for merchants. | [optional]
**prep_time_duration** | Option<**String**> | The duration of time it takes to prepare this fulfillment. The duration must be in RFC 3339 format (for example, \"P1W3D\"). | [optional]
**note** | Option<**String**> | A note to provide additional instructions about the pickup fulfillment displayed in the Square Point of Sale application and set by the API. | [optional]
**placed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was placed. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**accepted_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was marked in progress. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**rejected_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was rejected. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**ready_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment is marked as ready for pickup. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**expired_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment expired. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**picked_up_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was picked up by the recipient. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**canceled_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was canceled. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**cancel_reason** | Option<**String**> | A description of why the pickup was canceled. The maximum length: 100 characters. | [optional]
**is_curbside_pickup** | Option<**bool**> | If set to `true`, indicates that this pickup order is for curbside pickup, not in-store pickup. | [optional]
**curbside_pickup_details** | Option<[**models::FulfillmentPickupDetailsCurbsidePickupDetails**](FulfillmentPickupDetailsCurbsidePickupDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


