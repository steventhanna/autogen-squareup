# RetrieveTokenStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scopes** | Option<**Vec<String>**> | The list of scopes associated with an access token. | [optional]
**expires_at** | Option<**String**> | The date and time when the `access_token` expires, in RFC 3339 format. Empty if the token never expires. | [optional]
**client_id** | Option<**String**> | The Square-issued application ID associated with the access token. This is the same application ID used to obtain the token. | [optional]
**merchant_id** | Option<**String**> | The ID of the authorizing merchant's business. | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> |  Any errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


