# SearchTerminalActionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information on errors encountered during the request. | [optional]
**action** | Option<[**Vec<models::TerminalAction>**](TerminalAction.md)> | The requested search result of `TerminalAction`s. | [optional]
**cursor** | Option<**String**> | The pagination cursor to be used in a subsequent request. If empty, this is the final response.  See [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination) for more information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


