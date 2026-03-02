# BulkRetrieveTeamMemberBookingProfilesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_member_booking_profiles** | Option<[**std::collections::HashMap<String, models::RetrieveTeamMemberBookingProfileResponse>**](RetrieveTeamMemberBookingProfileResponse.md)> | The returned team members' booking profiles, as a map with `team_member_id` as the key and [TeamMemberBookingProfile](entity:TeamMemberBookingProfile) the value. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


