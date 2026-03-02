# Employee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | UUID for this object. | [optional]
**first_name** | Option<**String**> | The employee's first name. | [optional]
**last_name** | Option<**String**> | The employee's last name. | [optional]
**email** | Option<**String**> | The employee's email address | [optional]
**phone_number** | Option<**String**> | The employee's phone number in E.164 format, i.e. \"+12125554250\" | [optional]
**location_ids** | Option<**Vec<String>**> | A list of location IDs where this employee has access to. | [optional]
**status** | Option<[**models::EmployeeStatus**](EmployeeStatus.md)> |  | [optional]
**is_owner** | Option<**bool**> | Whether this employee is the owner of the merchant. Each merchant has one owner employee, and that employee has full authority over the account. | [optional]
**created_at** | Option<**String**> | A read-only timestamp in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | A read-only timestamp in RFC 3339 format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


