# \TeamApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_team_members**](TeamApi.md#bulk_create_team_members) | **POST** /v2/team-members/bulk-create | BulkCreateTeamMembers
[**bulk_update_team_members**](TeamApi.md#bulk_update_team_members) | **POST** /v2/team-members/bulk-update | BulkUpdateTeamMembers
[**create_job**](TeamApi.md#create_job) | **POST** /v2/team-members/jobs | CreateJob
[**create_team_member**](TeamApi.md#create_team_member) | **POST** /v2/team-members | CreateTeamMember
[**list_jobs**](TeamApi.md#list_jobs) | **GET** /v2/team-members/jobs | ListJobs
[**retrieve_job**](TeamApi.md#retrieve_job) | **GET** /v2/team-members/jobs/{job_id} | RetrieveJob
[**retrieve_team_member**](TeamApi.md#retrieve_team_member) | **GET** /v2/team-members/{team_member_id} | RetrieveTeamMember
[**retrieve_wage_setting**](TeamApi.md#retrieve_wage_setting) | **GET** /v2/team-members/{team_member_id}/wage-setting | RetrieveWageSetting
[**search_team_members**](TeamApi.md#search_team_members) | **POST** /v2/team-members/search | SearchTeamMembers
[**update_job**](TeamApi.md#update_job) | **PUT** /v2/team-members/jobs/{job_id} | UpdateJob
[**update_team_member**](TeamApi.md#update_team_member) | **PUT** /v2/team-members/{team_member_id} | UpdateTeamMember
[**update_wage_setting**](TeamApi.md#update_wage_setting) | **PUT** /v2/team-members/{team_member_id}/wage-setting | UpdateWageSetting



## bulk_create_team_members

> models::BulkCreateTeamMembersResponse bulk_create_team_members(bulk_create_team_members_request)
BulkCreateTeamMembers

Creates multiple `TeamMember` objects. The created `TeamMember` objects are returned on successful creates. This process is non-transactional and processes as much of the request as possible. If one of the creates in the request cannot be successfully processed, the request is not marked as failed, but the body of the response contains explicit error information for the failed create.  Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#bulk-create-team-members).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_create_team_members_request** | [**BulkCreateTeamMembersRequest**](BulkCreateTeamMembersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkCreateTeamMembersResponse**](BulkCreateTeamMembersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_team_members

> models::BulkUpdateTeamMembersResponse bulk_update_team_members(bulk_update_team_members_request)
BulkUpdateTeamMembers

Updates multiple `TeamMember` objects. The updated `TeamMember` objects are returned on successful updates. This process is non-transactional and processes as much of the request as possible. If one of the updates in the request cannot be successfully processed, the request is not marked as failed, but the body of the response contains explicit error information for the failed update. Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#bulk-update-team-members).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_update_team_members_request** | [**BulkUpdateTeamMembersRequest**](BulkUpdateTeamMembersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::BulkUpdateTeamMembersResponse**](BulkUpdateTeamMembersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_job

> models::CreateJobResponse create_job(create_job_request)
CreateJob

Creates a job in a seller account. A job defines a title and tip eligibility. Note that compensation is defined in a [job assignment](entity:JobAssignment) in a team member's wage setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_job_request** | [**CreateJobRequest**](CreateJobRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateJobResponse**](CreateJobResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team_member

> models::CreateTeamMemberResponse create_team_member(create_team_member_request)
CreateTeamMember

Creates a single `TeamMember` object. The `TeamMember` object is returned on successful creates. You must provide the following values in your request to this endpoint: - `given_name` - `family_name`  Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_team_member_request** | [**CreateTeamMemberRequest**](CreateTeamMemberRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateTeamMemberResponse**](CreateTeamMemberResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jobs

> models::ListJobsResponse list_jobs(cursor)
ListJobs

Lists jobs in a seller account. Results are sorted by title in ascending order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The pagination cursor returned by the previous call to this endpoint. Provide this cursor to retrieve the next page of results for your original request. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListJobsResponse**](ListJobsResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_job

> models::RetrieveJobResponse retrieve_job(job_id)
RetrieveJob

Retrieves a specified job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The ID of the job to retrieve. | [required] |

### Return type

[**models::RetrieveJobResponse**](RetrieveJobResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_team_member

> models::RetrieveTeamMemberResponse retrieve_team_member(team_member_id)
RetrieveTeamMember

Retrieves a `TeamMember` object for the given `TeamMember.id`. Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#retrieve-a-team-member).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_member_id** | **String** | The ID of the team member to retrieve. | [required] |

### Return type

[**models::RetrieveTeamMemberResponse**](RetrieveTeamMemberResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_wage_setting

> models::RetrieveWageSettingResponse retrieve_wage_setting(team_member_id)
RetrieveWageSetting

Retrieves a `WageSetting` object for a team member specified by `TeamMember.id`. For more information, see [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#retrievewagesetting).  Square recommends using [RetrieveTeamMember](api-endpoint:Team-RetrieveTeamMember) or [SearchTeamMembers](api-endpoint:Team-SearchTeamMembers) to get this information directly from the `TeamMember.wage_setting` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_member_id** | **String** | The ID of the team member for which to retrieve the wage setting. | [required] |

### Return type

[**models::RetrieveWageSettingResponse**](RetrieveWageSettingResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_team_members

> models::SearchTeamMembersResponse search_team_members(search_team_members_request)
SearchTeamMembers

Returns a paginated list of `TeamMember` objects for a business.  The list can be filtered by location IDs, `ACTIVE` or `INACTIVE` status, or whether the team member is the Square account owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_team_members_request** | [**SearchTeamMembersRequest**](SearchTeamMembersRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchTeamMembersResponse**](SearchTeamMembersResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_job

> models::UpdateJobResponse update_job(job_id, update_job_request)
UpdateJob

Updates the title or tip eligibility of a job. Changes to the title propagate to all `JobAssignment`, `Shift`, and `TeamMemberWage` objects that reference the job ID. Changes to tip eligibility propagate to all `TeamMemberWage` objects that reference the job ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The ID of the job to update. | [required] |
**update_job_request** | [**UpdateJobRequest**](UpdateJobRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateJobResponse**](UpdateJobResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team_member

> models::UpdateTeamMemberResponse update_team_member(team_member_id, update_team_member_request)
UpdateTeamMember

Updates a single `TeamMember` object. The `TeamMember` object is returned on successful updates. Learn about [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#update-a-team-member).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_member_id** | **String** | The ID of the team member to update. | [required] |
**update_team_member_request** | [**UpdateTeamMemberRequest**](UpdateTeamMemberRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateTeamMemberResponse**](UpdateTeamMemberResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wage_setting

> models::UpdateWageSettingResponse update_wage_setting(team_member_id, update_wage_setting_request)
UpdateWageSetting

Creates or updates a `WageSetting` object. The object is created if a `WageSetting` with the specified `team_member_id` doesn't exist. Otherwise, it fully replaces the `WageSetting` object for the team member. The `WageSetting` is returned on a successful update. For more information, see [Troubleshooting the Team API](https://developer.squareup.com/docs/team/troubleshooting#create-or-update-a-wage-setting).  Square recommends using [CreateTeamMember](api-endpoint:Team-CreateTeamMember) or [UpdateTeamMember](api-endpoint:Team-UpdateTeamMember) to manage the `TeamMember.wage_setting` field directly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_member_id** | **String** | The ID of the team member for which to update the `WageSetting` object. | [required] |
**update_wage_setting_request** | [**UpdateWageSettingRequest**](UpdateWageSettingRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateWageSettingResponse**](UpdateWageSettingResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

