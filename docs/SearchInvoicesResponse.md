# SearchInvoicesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoices** | Option<[**Vec<models::Invoice>**](Invoice.md)> | The list of invoices returned by the search. | [optional]
**cursor** | Option<**String**> | When a response is truncated, it includes a cursor that you can use in a  subsequent request to fetch the next set of invoices. If empty, this is the final  response.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information about errors encountered during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


