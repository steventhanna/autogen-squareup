# SearchEventsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information on errors encountered during the request. | [optional]
**events** | Option<[**Vec<models::Event>**](Event.md)> | The list of [Event](entity:Event)s returned by the search. | [optional]
**metadata** | Option<[**Vec<models::EventMetadata>**](EventMetadata.md)> | Contains the metadata of an event. For more information, see [Event](entity:Event). | [optional]
**cursor** | Option<**String**> | When a response is truncated, it includes a cursor that you can use in a subsequent request to fetch the next set of events. If empty, this is the final response.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


