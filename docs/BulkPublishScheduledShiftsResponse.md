# BulkPublishScheduledShiftsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**responses** | Option<[**std::collections::HashMap<String, models::PublishScheduledShiftResponse>**](PublishScheduledShiftResponse.md)> | A map of key-value pairs that represent responses for individual publish requests. The order of responses might differ from the order in which the requests were provided.  - Each key is the scheduled shift ID that was specified for a publish request. - Each value is the corresponding response. If the request succeeds, the value is the published scheduled shift. If the request fails, the value is an `errors` array containing any errors that occurred while processing the request. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any top-level errors that prevented the bulk operation from succeeding. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


