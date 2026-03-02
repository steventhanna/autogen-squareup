# Channel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The channel's unique ID. | [optional][readonly]
**merchant_id** | Option<**String**> | The unique ID of the merchant this channel belongs to. | [optional][readonly]
**name** | Option<**String**> | The name of the channel. | [optional]
**version** | Option<**i32**> | The version number which is incremented each time an update is made to the channel. | [optional][readonly]
**reference** | Option<[**models::Reference**](Reference.md)> |  | [optional]
**status** | Option<[**models::ChannelStatus**](ChannelStatus.md)> |  | [optional]
**created_at** | Option<**String**> | The timestamp for when the channel was created, in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). For more information, see [Working with Dates](https://developer.squareup.com/docs/build-basics/working-with-dates). | [optional][readonly]
**updated_at** | Option<**String**> | The timestamp for when the channel was last updated, in RFC 3339 format (for example, \"2016-09-04T23:59:33.123Z\"). For more information, see [Working with Dates](https://developer.squareup.com/docs/build-basics/working-with-dates). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


