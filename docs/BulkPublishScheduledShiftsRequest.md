# BulkPublishScheduledShiftsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheduled_shifts** | [**std::collections::HashMap<String, models::BulkPublishScheduledShiftsData>**](BulkPublishScheduledShiftsData.md) | A map of 1 to 100 key-value pairs that represent individual publish requests.  - Each key is the ID of a scheduled shift you want to publish. - Each value is a `BulkPublishScheduledShiftsData` object that contains the `version` field or is an empty object. | 
**scheduled_shift_notification_audience** | Option<[**models::ScheduledShiftNotificationAudience**](ScheduledShiftNotificationAudience.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


