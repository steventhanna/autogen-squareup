# BulkRetrieveChannelsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information about errors encountered during the request. | [optional]
**responses** | Option<[**std::collections::HashMap<String, models::RetrieveChannelResponse>**](RetrieveChannelResponse.md)> | A map of channel IDs to channel responses which tell whether retrieval for a specific channel is success or not. Channel response of a success retrieval would contain channel info whereas channel response of a failed retrieval would have error info. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


