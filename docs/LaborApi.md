# \LaborApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_publish_scheduled_shifts**](LaborApi.md#bulk_publish_scheduled_shifts) | **POST** /v2/labor/scheduled-shifts/bulk-publish | BulkPublishScheduledShifts
[**create_break_type**](LaborApi.md#create_break_type) | **POST** /v2/labor/break-types | CreateBreakType
[**create_scheduled_shift**](LaborApi.md#create_scheduled_shift) | **POST** /v2/labor/scheduled-shifts | CreateScheduledShift
[**create_shift**](LaborApi.md#create_shift) | **POST** /v2/labor/shifts | CreateShift
[**create_timecard**](LaborApi.md#create_timecard) | **POST** /v2/labor/timecards | CreateTimecard
[**delete_break_type**](LaborApi.md#delete_break_type) | **DELETE** /v2/labor/break-types/{id} | DeleteBreakType
[**delete_shift**](LaborApi.md#delete_shift) | **DELETE** /v2/labor/shifts/{id} | DeleteShift
[**delete_timecard**](LaborApi.md#delete_timecard) | **DELETE** /v2/labor/timecards/{id} | DeleteTimecard
[**get_break_type**](LaborApi.md#get_break_type) | **GET** /v2/labor/break-types/{id} | GetBreakType
[**get_employee_wage**](LaborApi.md#get_employee_wage) | **GET** /v2/labor/employee-wages/{id} | GetEmployeeWage
[**get_shift**](LaborApi.md#get_shift) | **GET** /v2/labor/shifts/{id} | GetShift
[**get_team_member_wage**](LaborApi.md#get_team_member_wage) | **GET** /v2/labor/team-member-wages/{id} | GetTeamMemberWage
[**list_break_types**](LaborApi.md#list_break_types) | **GET** /v2/labor/break-types | ListBreakTypes
[**list_employee_wages**](LaborApi.md#list_employee_wages) | **GET** /v2/labor/employee-wages | ListEmployeeWages
[**list_team_member_wages**](LaborApi.md#list_team_member_wages) | **GET** /v2/labor/team-member-wages | ListTeamMemberWages
[**list_workweek_configs**](LaborApi.md#list_workweek_configs) | **GET** /v2/labor/workweek-configs | ListWorkweekConfigs
[**publish_scheduled_shift**](LaborApi.md#publish_scheduled_shift) | **POST** /v2/labor/scheduled-shifts/{id}/publish | PublishScheduledShift
[**retrieve_scheduled_shift**](LaborApi.md#retrieve_scheduled_shift) | **GET** /v2/labor/scheduled-shifts/{id} | RetrieveScheduledShift
[**retrieve_timecard**](LaborApi.md#retrieve_timecard) | **GET** /v2/labor/timecards/{id} | RetrieveTimecard
[**search_scheduled_shifts**](LaborApi.md#search_scheduled_shifts) | **POST** /v2/labor/scheduled-shifts/search | SearchScheduledShifts
[**search_shifts**](LaborApi.md#search_shifts) | **POST** /v2/labor/shifts/search | SearchShifts
[**search_timecards**](LaborApi.md#search_timecards) | **POST** /v2/labor/timecards/search | SearchTimecards
[**update_break_type**](LaborApi.md#update_break_type) | **PUT** /v2/labor/break-types/{id} | UpdateBreakType
[**update_scheduled_shift**](LaborApi.md#update_scheduled_shift) | **PUT** /v2/labor/scheduled-shifts/{id} | UpdateScheduledShift
[**update_shift**](LaborApi.md#update_shift) | **PUT** /v2/labor/shifts/{id} | UpdateShift
[**update_timecard**](LaborApi.md#update_timecard) | **PUT** /v2/labor/timecards/{id} | UpdateTimecard
[**update_workweek_config**](LaborApi.md#update_workweek_config) | **PUT** /v2/labor/workweek-configs/{id} | UpdateWorkweekConfig



## bulk_publish_scheduled_shifts

> models::BulkPublishScheduledShiftsResponse bulk_publish_scheduled_shifts(bulk_publish_scheduled_shifts_request)
BulkPublishScheduledShifts

Publishes 1 - 100 scheduled shifts. This endpoint takes a map of individual publish requests and returns a map of responses. When a scheduled shift is published, Square keeps the `draft_shift_details` field as is and copies it to the `published_shift_details` field.  The minimum `start_at` and maximum `end_at` timestamps of all shifts in a `BulkPublishScheduledShifts` request must fall within a two-week period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_publish_scheduled_shifts_request** | [**BulkPublishScheduledShiftsRequest**](BulkPublishScheduledShiftsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkPublishScheduledShiftsResponse**](BulkPublishScheduledShiftsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_break_type

> models::CreateBreakTypeResponse create_break_type(create_break_type_request)
CreateBreakType

Creates a new `BreakType`.  A `BreakType` is a template for creating `Break` objects. You must provide the following values in your request to this endpoint:  - `location_id` - `break_name` - `expected_duration` - `is_paid`  You can only have three `BreakType` instances per location. If you attempt to add a fourth `BreakType` for a location, an `INVALID_REQUEST_ERROR` \"Exceeded limit of 3 breaks per location.\" is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_break_type_request** | [**CreateBreakTypeRequest**](CreateBreakTypeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateBreakTypeResponse**](CreateBreakTypeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_scheduled_shift

> models::CreateScheduledShiftResponse create_scheduled_shift(create_scheduled_shift_request)
CreateScheduledShift

Creates a scheduled shift by providing draft shift details such as job ID, team member assignment, and start and end times.  The following `draft_shift_details` fields are required: - `location_id` - `job_id` - `start_at` - `end_at`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_scheduled_shift_request** | [**CreateScheduledShiftRequest**](CreateScheduledShiftRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateScheduledShiftResponse**](CreateScheduledShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_shift

> models::CreateShiftResponse create_shift(create_shift_request)
CreateShift

Creates a new `Shift`.  A `Shift` represents a complete workday for a single team member. You must provide the following values in your request to this endpoint:  - `location_id` - `team_member_id` - `start_at`  An attempt to create a new `Shift` can result in a `BAD_REQUEST` error when: - The `status` of the new `Shift` is `OPEN` and the team member has another shift with an `OPEN` status. - The `start_at` date is in the future. - The `start_at` or `end_at` date overlaps another shift for the same team member. - The `Break` instances are set in the request and a break `start_at` is before the `Shift.start_at`, a break `end_at` is after the `Shift.end_at`, or both.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_shift_request** | [**CreateShiftRequest**](CreateShiftRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateShiftResponse**](CreateShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_timecard

> models::CreateTimecardResponse create_timecard(create_timecard_request)
CreateTimecard

Creates a new `Timecard`.  A `Timecard` represents a complete workday for a single team member. You must provide the following values in your request to this endpoint:  - `location_id` - `team_member_id` - `start_at`  An attempt to create a new `Timecard` can result in a `BAD_REQUEST` error when: - The `status` of the new `Timecard` is `OPEN` and the team member has another timecard with an `OPEN` status. - The `start_at` date is in the future. - The `start_at` or `end_at` date overlaps another timecard for the same team member. - The `Break` instances are set in the request and a break `start_at` is before the `Timecard.start_at`, a break `end_at` is after the `Timecard.end_at`, or both.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_timecard_request** | [**CreateTimecardRequest**](CreateTimecardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateTimecardResponse**](CreateTimecardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_break_type

> models::DeleteBreakTypeResponse delete_break_type(id)
DeleteBreakType

Deletes an existing `BreakType`.  A `BreakType` can be deleted even if it is referenced from a `Shift`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `BreakType` being deleted. | [required] |

### Return type

[**models::DeleteBreakTypeResponse**](DeleteBreakTypeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_shift

> models::DeleteShiftResponse delete_shift(id)
DeleteShift

Deletes a `Shift`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `Shift` being deleted. | [required] |

### Return type

[**models::DeleteShiftResponse**](DeleteShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_timecard

> models::DeleteTimecardResponse delete_timecard(id)
DeleteTimecard

Deletes a `Timecard`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `Timecard` being deleted. | [required] |

### Return type

[**models::DeleteTimecardResponse**](DeleteTimecardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_break_type

> models::GetBreakTypeResponse get_break_type(id)
GetBreakType

Returns a single `BreakType` specified by `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `BreakType` being retrieved. | [required] |

### Return type

[**models::GetBreakTypeResponse**](GetBreakTypeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_employee_wage

> models::GetEmployeeWageResponse get_employee_wage(id)
GetEmployeeWage

Returns a single `EmployeeWage` specified by `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `EmployeeWage` being retrieved. | [required] |

### Return type

[**models::GetEmployeeWageResponse**](GetEmployeeWageResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shift

> models::GetShiftResponse get_shift(id)
GetShift

Returns a single `Shift` specified by `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `Shift` being retrieved. | [required] |

### Return type

[**models::GetShiftResponse**](GetShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_member_wage

> models::GetTeamMemberWageResponse get_team_member_wage(id)
GetTeamMemberWage

Returns a single `TeamMemberWage` specified by `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `TeamMemberWage` being retrieved. | [required] |

### Return type

[**models::GetTeamMemberWageResponse**](GetTeamMemberWageResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_break_types

> models::ListBreakTypesResponse list_break_types(location_id, limit, cursor)
ListBreakTypes

Returns a paginated list of `BreakType` instances for a business.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | Option<**String**> | Filter the returned `BreakType` results to only those that are associated with the specified location. |  |
**limit** | Option<**i32**> | The maximum number of `BreakType` results to return per page. The number can range between 1 and 200. The default is 200. |  |
**cursor** | Option<**String**> | A pointer to the next page of `BreakType` results to fetch. |  |

### Return type

[**models::ListBreakTypesResponse**](ListBreakTypesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_employee_wages

> models::ListEmployeeWagesResponse list_employee_wages(employee_id, limit, cursor)
ListEmployeeWages

Returns a paginated list of `EmployeeWage` instances for a business.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | Option<**String**> | Filter the returned wages to only those that are associated with the specified employee. |  |
**limit** | Option<**i32**> | The maximum number of `EmployeeWage` results to return per page. The number can range between 1 and 200. The default is 200. |  |
**cursor** | Option<**String**> | A pointer to the next page of `EmployeeWage` results to fetch. |  |

### Return type

[**models::ListEmployeeWagesResponse**](ListEmployeeWagesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_team_member_wages

> models::ListTeamMemberWagesResponse list_team_member_wages(team_member_id, limit, cursor)
ListTeamMemberWages

Returns a paginated list of `TeamMemberWage` instances for a business.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_member_id** | Option<**String**> | Filter the returned wages to only those that are associated with the specified team member. |  |
**limit** | Option<**i32**> | The maximum number of `TeamMemberWage` results to return per page. The number can range between 1 and 200. The default is 200. |  |
**cursor** | Option<**String**> | A pointer to the next page of `EmployeeWage` results to fetch. |  |

### Return type

[**models::ListTeamMemberWagesResponse**](ListTeamMemberWagesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workweek_configs

> models::ListWorkweekConfigsResponse list_workweek_configs(limit, cursor)
ListWorkweekConfigs

Returns a list of `WorkweekConfig` instances for a business.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The maximum number of `WorkweekConfigs` results to return per page. |  |
**cursor** | Option<**String**> | A pointer to the next page of `WorkweekConfig` results to fetch. |  |

### Return type

[**models::ListWorkweekConfigsResponse**](ListWorkweekConfigsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_scheduled_shift

> models::PublishScheduledShiftResponse publish_scheduled_shift(id, publish_scheduled_shift_request)
PublishScheduledShift

Publishes a scheduled shift. When a scheduled shift is published, Square keeps the `draft_shift_details` field as is and copies it to the `published_shift_details` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the scheduled shift to publish. | [required] |
**publish_scheduled_shift_request** | [**PublishScheduledShiftRequest**](PublishScheduledShiftRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::PublishScheduledShiftResponse**](PublishScheduledShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_scheduled_shift

> models::RetrieveScheduledShiftResponse retrieve_scheduled_shift(id)
RetrieveScheduledShift

Retrieves a scheduled shift by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the scheduled shift to retrieve. | [required] |

### Return type

[**models::RetrieveScheduledShiftResponse**](RetrieveScheduledShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_timecard

> models::RetrieveTimecardResponse retrieve_timecard(id)
RetrieveTimecard

Returns a single `Timecard` specified by `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `Timecard` being retrieved. | [required] |

### Return type

[**models::RetrieveTimecardResponse**](RetrieveTimecardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_scheduled_shifts

> models::SearchScheduledShiftsResponse search_scheduled_shifts(search_scheduled_shifts_request)
SearchScheduledShifts

Returns a paginated list of scheduled shifts, with optional filter and sort settings. By default, results are sorted by `start_at` in ascending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_scheduled_shifts_request** | [**SearchScheduledShiftsRequest**](SearchScheduledShiftsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchScheduledShiftsResponse**](SearchScheduledShiftsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_shifts

> models::SearchShiftsResponse search_shifts(search_shifts_request)
SearchShifts

Returns a paginated list of `Shift` records for a business. The list to be returned can be filtered by: - Location IDs - Team member IDs - Shift status (`OPEN` or `CLOSED`) - Shift start - Shift end - Workday details  The list can be sorted by: - `START_AT` - `END_AT` - `CREATED_AT` - `UPDATED_AT`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_shifts_request** | [**SearchShiftsRequest**](SearchShiftsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchShiftsResponse**](SearchShiftsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_timecards

> models::SearchTimecardsResponse search_timecards(search_timecards_request)
SearchTimecards

Returns a paginated list of `Timecard` records for a business. The list to be returned can be filtered by: - Location IDs - Team member IDs - Timecard status (`OPEN` or `CLOSED`) - Timecard start - Timecard end - Workday details  The list can be sorted by: - `START_AT` - `END_AT` - `CREATED_AT` - `UPDATED_AT`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_timecards_request** | [**SearchTimecardsRequest**](SearchTimecardsRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchTimecardsResponse**](SearchTimecardsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_break_type

> models::UpdateBreakTypeResponse update_break_type(id, update_break_type_request)
UpdateBreakType

Updates an existing `BreakType`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  The UUID for the `BreakType` being updated. | [required] |
**update_break_type_request** | [**UpdateBreakTypeRequest**](UpdateBreakTypeRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateBreakTypeResponse**](UpdateBreakTypeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scheduled_shift

> models::UpdateScheduledShiftResponse update_scheduled_shift(id, update_scheduled_shift_request)
UpdateScheduledShift

Updates the draft shift details for a scheduled shift. This endpoint supports sparse updates, so only new, changed, or removed fields are required in the request. You must publish the shift to make updates public.  You can make the following updates to `draft_shift_details`: - Change the `location_id`, `job_id`, `start_at`, and `end_at` fields. - Add, change, or clear the `team_member_id` and `notes` fields. To clear these fields, set the value to null. - Change the `is_deleted` field. To delete a scheduled shift, set `is_deleted` to true and then publish the shift.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the scheduled shift to update. | [required] |
**update_scheduled_shift_request** | [**UpdateScheduledShiftRequest**](UpdateScheduledShiftRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateScheduledShiftResponse**](UpdateScheduledShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_shift

> models::UpdateShiftResponse update_shift(id, update_shift_request)
UpdateShift

Updates an existing `Shift`.  When adding a `Break` to a `Shift`, any earlier `Break` instances in the `Shift` have the `end_at` property set to a valid RFC-3339 datetime string.  When closing a `Shift`, all `Break` instances in the `Shift` must be complete with `end_at` set on each `Break`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object being updated. | [required] |
**update_shift_request** | [**UpdateShiftRequest**](UpdateShiftRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateShiftResponse**](UpdateShiftResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_timecard

> models::UpdateTimecardResponse update_timecard(id, update_timecard_request)
UpdateTimecard

Updates an existing `Timecard`.  When adding a `Break` to a `Timecard`, any earlier `Break` instances in the `Timecard` have the `end_at` property set to a valid RFC-3339 datetime string.  When closing a `Timecard`, all `Break` instances in the `Timecard` must be complete with `end_at` set on each `Break`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the object being updated. | [required] |
**update_timecard_request** | [**UpdateTimecardRequest**](UpdateTimecardRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateTimecardResponse**](UpdateTimecardResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workweek_config

> models::UpdateWorkweekConfigResponse update_workweek_config(id, update_workweek_config_request)
UpdateWorkweekConfig

Updates a `WorkweekConfig`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The UUID for the `WorkweekConfig` object being updated. | [required] |
**update_workweek_config_request** | [**UpdateWorkweekConfigRequest**](UpdateWorkweekConfigRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateWorkweekConfigResponse**](UpdateWorkweekConfigResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

