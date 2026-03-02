# UpdateItemTaxesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_ids** | **Vec<String>** | IDs for the CatalogItems associated with the CatalogTax objects being updated. No more than 1,000 IDs may be provided. | 
**taxes_to_enable** | Option<**Vec<String>**> | IDs of the CatalogTax objects to enable. At least one of `taxes_to_enable` or `taxes_to_disable` must be specified. | [optional]
**taxes_to_disable** | Option<**Vec<String>**> | IDs of the CatalogTax objects to disable. At least one of `taxes_to_enable` or `taxes_to_disable` must be specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


