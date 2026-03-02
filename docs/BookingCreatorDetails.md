# BookingCreatorDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creator_type** | Option<[**models::BookingCreatorDetailsCreatorType**](BookingCreatorDetailsCreatorType.md)> |  | [optional]
**team_member_id** | Option<**String**> | The ID of the team member who created the booking, when the booking creator is of the `TEAM_MEMBER` type. Access to this field requires seller-level permissions. | [optional][readonly]
**customer_id** | Option<**String**> | The ID of the customer who created the booking, when the booking creator is of the `CUSTOMER` type. Access to this field requires seller-level permissions. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


