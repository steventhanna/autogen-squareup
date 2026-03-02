# \EmployeesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_employees**](EmployeesApi.md#list_employees) | **GET** /v2/employees | ListEmployees
[**retrieve_employee**](EmployeesApi.md#retrieve_employee) | **GET** /v2/employees/{id} | RetrieveEmployee



## list_employees

> models::ListEmployeesResponse list_employees(location_id, status, limit, cursor)
ListEmployees



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | Option<**String**> |  |  |
**status** | Option<[**EmployeeStatus**](.md)> | Specifies the EmployeeStatus to filter the employee by. |  |
**limit** | Option<**i32**> | The number of employees to be returned on each page. |  |
**cursor** | Option<**String**> | The token required to retrieve the specified page of results. |  |

### Return type

[**models::ListEmployeesResponse**](ListEmployeesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_employee

> models::RetrieveEmployeeResponse retrieve_employee(id)
RetrieveEmployee



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | UUID for the employee that was requested. | [required] |

### Return type

[**models::RetrieveEmployeeResponse**](RetrieveEmployeeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

