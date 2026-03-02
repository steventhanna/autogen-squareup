# ListCustomerCustomAttributeDefinitionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_attribute_definitions** | Option<[**Vec<models::CustomAttributeDefinition>**](CustomAttributeDefinition.md)> | The retrieved custom attribute definitions. If no custom attribute definitions are found, Square returns an empty object (`{}`). | [optional]
**cursor** | Option<**String**> | The cursor to provide in your next call to this endpoint to retrieve the next page of results for your original request. This field is present only if the request succeeded and additional results are available. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). | [optional]
**errors** | Option<[**Vec<models::Error>**](Error.md)> | Any errors that occurred during the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


