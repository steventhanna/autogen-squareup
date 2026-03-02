# FulfillmentShipmentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipient** | Option<[**models::FulfillmentRecipient**](FulfillmentRecipient.md)> |  | [optional]
**carrier** | Option<**String**> | The shipping carrier being used to ship this fulfillment (such as UPS, FedEx, or USPS). | [optional]
**shipping_note** | Option<**String**> | A note with additional information for the shipping carrier. | [optional]
**shipping_type** | Option<**String**> | A description of the type of shipping product purchased from the carrier (such as First Class, Priority, or Express). | [optional]
**tracking_number** | Option<**String**> | The reference number provided by the carrier to track the shipment's progress. | [optional]
**tracking_url** | Option<**String**> | A link to the tracking webpage on the carrier's website. | [optional]
**placed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the shipment was requested. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**in_progress_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment was moved to the `RESERVED` state, which  indicates that preparation of this shipment has begun. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**packaged_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment was moved to the `PREPARED` state, which indicates that the fulfillment is packaged. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**expected_shipped_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the shipment is expected to be delivered to the shipping carrier. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**shipped_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment was moved to the `COMPLETED` state, which indicates that the fulfillment has been given to the shipping carrier. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**canceled_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating the shipment was canceled. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**cancel_reason** | Option<**String**> | A description of why the shipment was canceled. | [optional]
**failed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the shipment failed to be completed. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**failure_reason** | Option<**String**> | A description of why the shipment failed to be completed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


