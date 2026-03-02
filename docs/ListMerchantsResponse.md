# ListMerchantsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Information on errors encountered during the request. | [optional]
**merchant** | Option<[**Vec<models::Merchant>**](Merchant.md)> | The requested `Merchant` entities. | [optional]
**cursor** | Option<**i32**> | If the  response is truncated, the cursor to use in next  request to fetch next set of objects. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


