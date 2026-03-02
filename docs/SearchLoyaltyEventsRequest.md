# SearchLoyaltyEventsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<[**models::LoyaltyEventQuery**](LoyaltyEventQuery.md)> |  | [optional]
**limit** | Option<**i32**> | The maximum number of results to include in the response.  The last page might contain fewer events.  The default is 30 events. | [optional]
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this to retrieve the next set of results for your original query. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


