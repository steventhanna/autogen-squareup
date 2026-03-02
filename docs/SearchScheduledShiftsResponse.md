# SearchScheduledShiftsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheduled_shifts** | Option<[**Vec<models::ScheduledShift>**](ScheduledShift.md)> | A paginated list of scheduled shifts that match the query conditions. | [optional]
**cursor** | Option<**String**> | The pagination cursor used to retrieve the next page of results. This field is present only if additional results are available. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


