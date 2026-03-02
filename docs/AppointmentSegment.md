# AppointmentSegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration_minutes** | Option<**i32**> | The time span in minutes of an appointment segment. | [optional]
**service_variation_id** | Option<**String**> | The ID of the [CatalogItemVariation](entity:CatalogItemVariation) object representing the service booked in this segment. | [optional]
**team_member_id** | **String** | The ID of the [TeamMember](entity:TeamMember) object representing the team member booked in this segment. | 
**service_variation_version** | Option<**i64**> | The current version of the item variation representing the service booked in this segment. | [optional]
**intermission_minutes** | Option<**i32**> | Time between the end of this segment and the beginning of the subsequent segment. | [optional][readonly]
**any_team_member** | Option<**bool**> | Whether the customer accepts any team member, instead of a specific one, to serve this segment. | [optional][readonly]
**resource_ids** | Option<**Vec<String>**> | The IDs of the seller-accessible resources used for this appointment segment. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


