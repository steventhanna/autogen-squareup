# UpdateInvoiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice** | [**models::Invoice**](Invoice.md) |  | 
**idempotency_key** | Option<**String**> | A unique string that identifies the `UpdateInvoice` request. If you do not provide `idempotency_key` (or provide an empty string as the value), the endpoint treats each request as independent.  For more information, see [Idempotency](https://developer.squareup.com/docs/build-basics/common-api-patterns/idempotency). | [optional]
**fields_to_clear** | Option<**Vec<String>**> | The list of fields to clear. Although this field is currently supported, we recommend using null values or the `remove` field when possible. For examples, see [Update an Invoice](https://developer.squareup.com/docs/invoices-api/update-invoices). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


