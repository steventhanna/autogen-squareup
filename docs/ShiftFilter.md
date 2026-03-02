# ShiftFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**location_ids** | Option<**Vec<String>**> | Fetch shifts for the specified location. | [optional]
**employee_ids** | Option<**Vec<String>**> | Fetch shifts for the specified employees. DEPRECATED at version 2020-08-26. Use `team_member_ids` instead. | [optional]
**status** | Option<[**models::ShiftFilterStatus**](ShiftFilterStatus.md)> |  | [optional]
**start** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**end** | Option<[**models::TimeRange**](TimeRange.md)> |  | [optional]
**workday** | Option<[**models::ShiftWorkday**](ShiftWorkday.md)> |  | [optional]
**team_member_ids** | Option<**Vec<String>**> | Fetch shifts for the specified team members. Replaced `employee_ids` at version \"2020-08-26\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


