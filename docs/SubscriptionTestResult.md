# SubscriptionTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A Square-generated unique ID for the subscription test result. | [optional][readonly]
**status_code** | Option<**i32**> | The status code returned by the subscription notification URL. | [optional]
**payload** | Option<**String**> | An object containing the payload of the test event. For example, a `payment.created` event. | [optional]
**created_at** | Option<**String**> | The timestamp of when the subscription was created, in RFC 3339 format.  For example, \"2016-09-04T23:59:33.123Z\". | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp of when the subscription was updated, in RFC 3339 format. For example, \"2016-09-04T23:59:33.123Z\". Because a subscription test result is unique, this field is the same as the `created_at` field. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


