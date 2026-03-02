# TeamMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID for the team member. | [optional][readonly]
**reference_id** | Option<**String**> | A second ID used to associate the team member with an entity in another system. | [optional]
**is_owner** | Option<**bool**> | Whether the team member is the owner of the Square account. | [optional][readonly]
**status** | Option<[**models::TeamMemberStatus**](TeamMemberStatus.md)> |  | [optional]
**given_name** | Option<**String**> | The given name (that is, the first name) associated with the team member. | [optional]
**family_name** | Option<**String**> | The family name (that is, the last name) associated with the team member. | [optional]
**email_address** | Option<**String**> | The email address associated with the team member. After accepting the invitation from Square, only the team member can change this value. | [optional]
**phone_number** | Option<**String**> | The team member's phone number, in E.164 format. For example: +14155552671 - the country code is 1 for US +551155256325 - the country code is 55 for BR | [optional]
**created_at** | Option<**String**> | The timestamp when the team member was created, in RFC 3339 format. | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp when the team member was last updated, in RFC 3339 format. | [optional][readonly]
**assigned_locations** | Option<[**models::TeamMemberAssignedLocations**](TeamMemberAssignedLocations.md)> |  | [optional]
**wage_setting** | Option<[**models::WageSetting**](WageSetting.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


