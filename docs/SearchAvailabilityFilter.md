# SearchAvailabilityFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_at_range** | [**models::TimeRange**](TimeRange.md) |  | 
**location_id** | Option<**String**> | The query expression to search for buyer-accessible availabilities with their location IDs matching the specified location ID. This query expression cannot be set if `booking_id` is set. | [optional]
**segment_filters** | Option<[**Vec<models::SegmentFilter>**](SegmentFilter.md)> | The query expression to search for buyer-accessible availabilities matching the specified list of segment filters. If the size of the `segment_filters` list is `n`, the search returns availabilities with `n` segments per availability.  This query expression cannot be set if `booking_id` is set. | [optional]
**booking_id** | Option<**String**> | The query expression to search for buyer-accessible availabilities for an existing booking by matching the specified `booking_id` value. This is commonly used to reschedule an appointment. If this expression is set, the `location_id` and `segment_filters` expressions cannot be set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


