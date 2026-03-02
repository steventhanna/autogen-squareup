# LaborTimecardDeletedEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**merchant_id** | Option<**String**> | The ID of the merchant associated with the event. | [optional]
**r#type** | Option<**String**> | The type of event. For this event, the value is `labor.timecard.deleted`. | [optional]
**event_id** | Option<**String**> | The unique ID for the event. | [optional]
**created_at** | Option<**String**> | The timestamp of when the event was created, in RFC 3339 format. | [optional][readonly]
**data** | Option<[**models::LaborTimecardDeletedEventData**](LaborTimecardDeletedEventData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


