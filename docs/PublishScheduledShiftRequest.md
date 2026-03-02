# PublishScheduledShiftRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | **String** | A unique identifier for the `PublishScheduledShift` request, used to ensure the [idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency) of the operation. | 
**version** | Option<**i32**> | The current version of the scheduled shift, used to enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control. If the provided version doesn't match the server version, the request fails. If omitted, Square executes a blind write, potentially overwriting data from another publish request. | [optional]
**scheduled_shift_notification_audience** | Option<[**models::ScheduledShiftNotificationAudience**](ScheduledShiftNotificationAudience.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


