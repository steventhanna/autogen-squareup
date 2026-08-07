# FulfillmentInStoreDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**note** | Option<**String**> | A note to provide additional instructions about the in-store fulfillment displayed in the Square Point of Sale application and set by the API. | [optional]
**recipient** | Option<[**models::FulfillmentRecipient**](FulfillmentRecipient.md)> |  | [optional]
**placed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was placed. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**completed_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was completed. This field is automatically set when the fulfillment `state` changes to `COMPLETED`. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**in_progress_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicates when the seller started processing the fulfillment. This field is automatically set when the fulfillment `state` changes to `RESERVED`. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**prepared_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was moved to the `PREPARED` state, which indicates that the fulfillment is ready. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]
**canceled_at** | Option<**String**> | The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was canceled. This field is automatically set when the fulfillment `state` changes to `CANCELED`. The timestamp must be in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


