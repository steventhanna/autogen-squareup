# ListTeamMemberBookingProfilesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**team_member_booking_profiles** | Option<[**Vec<models::TeamMemberBookingProfile>**](TeamMemberBookingProfile.md)> | The list of team member booking profiles. The results are returned in the ascending order of the time when the team member booking profiles were last updated. Multiple booking profiles updated at the same time are further sorted in the ascending order of their IDs. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in the subsequent request to get the next page of the results. Stop retrieving the next page of the results when the cursor is not set. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


