# ScheduledShiftWorkday

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_range** | Option<[**models::DateRange**](DateRange.md)> |  | [optional]
**match_scheduled_shifts_by** | Option<[**models::ScheduledShiftWorkdayMatcher**](ScheduledShiftWorkdayMatcher.md)> |  | [optional]
**default_timezone** | Option<**String**> | Location-specific timezones convert workdays to datetime filters. Every location included in the query must have a timezone or this field must be provided as a fallback. Format: the IANA timezone database identifier for the relevant timezone. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


