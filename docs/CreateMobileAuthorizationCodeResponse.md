# CreateMobileAuthorizationCodeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorization_code** | Option<**String**> | The generated authorization code that connects a mobile application instance to a Square account. | [optional]
**expires_at** | Option<**String**> | The timestamp when `authorization_code` expires, in [RFC 3339](https://tools.ietf.org/html/rfc3339) format (for example, \"2016-09-04T23:59:33.123Z\"). | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


