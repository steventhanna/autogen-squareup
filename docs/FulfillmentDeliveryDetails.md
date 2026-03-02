# FulfillmentDeliveryDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipient** | Option<[**models::FulfillmentRecipient**](FulfillmentRecipient.md)> |  | [optional]
**schedule_type** | Option<[**models::FulfillmentDeliveryDetailsOrderFulfillmentDeliveryDetailsScheduleType**](FulfillmentDeliveryDetailsOrderFulfillmentDeliveryDetailsScheduleType.md)> |  | [optional]
**placed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was placed. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\").  Must be in RFC 3339 timestamp format, e.g., \"2016-09-04T23:59:33.123Z\". | [optional][readonly]
**deliver_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) that represents the start of the delivery period. When the fulfillment `schedule_type` is `ASAP`, the field is automatically set to the current time plus the `prep_time_duration`. Otherwise, the application can set this field while the fulfillment `state` is `PROPOSED`, `RESERVED`, or `PREPARED` (any time before the terminal state such as `COMPLETED`, `CANCELED`, and `FAILED`).  The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**prep_time_duration** | Option<**String**> | The duration of time it takes to prepare and deliver this fulfillment. The duration must be in RFC 3339 format (for example, \"P1W3D\"). | [optional]
**delivery_window_duration** | Option<**String**> | The time period after `deliver_at` in which to deliver the order. Applications can set this field when the fulfillment `state` is `PROPOSED`, `RESERVED`, or `PREPARED` (any time before the terminal state such as `COMPLETED`, `CANCELED`, and `FAILED`).  The duration must be in RFC 3339 format (for example, \"P1W3D\"). | [optional]
**note** | Option<**String**> | Provides additional instructions about the delivery fulfillment. It is displayed in the Square Point of Sale application and set by the API. | [optional]
**completed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicates when the seller completed the fulfillment. This field is automatically set when  fulfillment `state` changes to `COMPLETED`. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**in_progress_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicates when the seller started processing the fulfillment. This field is automatically set when the fulfillment `state` changes to `RESERVED`. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**rejected_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was rejected. This field is automatically set when the fulfillment `state` changes to `FAILED`. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**ready_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the seller marked the fulfillment as ready for courier pickup. This field is automatically set when the fulfillment `state` changes to PREPARED. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**delivered_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was delivered to the recipient. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**canceled_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was canceled. This field is automatically set when the fulfillment `state` changes to `CANCELED`.  The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**cancel_reason** | Option<**String**> | The delivery cancellation reason. Max length: 100 characters. | [optional]
**courier_pickup_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when an order can be picked up by the courier for delivery. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**courier_pickup_window_duration** | Option<**String**> | The time period after `courier_pickup_at` in which the courier should pick up the order. The duration must be in RFC 3339 format (for example, \"P1W3D\"). | [optional]
**is_no_contact_delivery** | Option<**bool**> | Whether the delivery is preferred to be no contact. | [optional]
**dropoff_notes** | Option<**String**> | A note to provide additional instructions about how to deliver the order. | [optional]
**courier_provider_name** | Option<**String**> | The name of the courier provider. | [optional]
**courier_support_phone_number** | Option<**String**> | The support phone number of the courier. | [optional]
**square_delivery_id** | Option<**String**> | The identifier for the delivery created by Square. | [optional]
**external_delivery_id** | Option<**String**> | The identifier for the delivery created by the third-party courier service. | [optional]
**managed_delivery** | Option<**bool**> | The flag to indicate the delivery is managed by a third party (ie DoorDash), which means we may not receive all recipient information for PII purposes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


