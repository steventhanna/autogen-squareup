# \InvoicesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_invoice**](InvoicesApi.md#cancel_invoice) | **POST** /v2/invoices/{invoice_id}/cancel | CancelInvoice
[**create_invoice**](InvoicesApi.md#create_invoice) | **POST** /v2/invoices | CreateInvoice
[**create_invoice_attachment**](InvoicesApi.md#create_invoice_attachment) | **POST** /v2/invoices/{invoice_id}/attachments | CreateInvoiceAttachment
[**delete_invoice**](InvoicesApi.md#delete_invoice) | **DELETE** /v2/invoices/{invoice_id} | DeleteInvoice
[**delete_invoice_attachment**](InvoicesApi.md#delete_invoice_attachment) | **DELETE** /v2/invoices/{invoice_id}/attachments/{attachment_id} | DeleteInvoiceAttachment
[**get_invoice**](InvoicesApi.md#get_invoice) | **GET** /v2/invoices/{invoice_id} | GetInvoice
[**list_invoices**](InvoicesApi.md#list_invoices) | **GET** /v2/invoices | ListInvoices
[**publish_invoice**](InvoicesApi.md#publish_invoice) | **POST** /v2/invoices/{invoice_id}/publish | PublishInvoice
[**search_invoices**](InvoicesApi.md#search_invoices) | **POST** /v2/invoices/search | SearchInvoices
[**update_invoice**](InvoicesApi.md#update_invoice) | **PUT** /v2/invoices/{invoice_id} | UpdateInvoice



## cancel_invoice

> models::CancelInvoiceResponse cancel_invoice(invoice_id, cancel_invoice_request)
CancelInvoice

Cancels an invoice. The seller cannot collect payments for  the canceled invoice.  You cannot cancel an invoice in the `DRAFT` state or in a terminal state: `PAID`, `REFUNDED`, `CANCELED`, or `FAILED`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the [invoice](entity:Invoice) to cancel. | [required] |
**cancel_invoice_request** | [**CancelInvoiceRequest**](CancelInvoiceRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CancelInvoiceResponse**](CancelInvoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_invoice

> models::CreateInvoiceResponse create_invoice(create_invoice_request)
CreateInvoice

Creates a draft [invoice](entity:Invoice)  for an order created using the Orders API.  A draft invoice remains in your account and no action is taken.  You must publish the invoice before Square can process it (send it to the customer's email address or charge the customer’s card on file).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_invoice_request** | [**CreateInvoiceRequest**](CreateInvoiceRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateInvoiceResponse**](CreateInvoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_invoice_attachment

> models::CreateInvoiceAttachmentResponse create_invoice_attachment(invoice_id, request, image_file)
CreateInvoiceAttachment

Uploads a file and attaches it to an invoice. This endpoint accepts HTTP multipart/form-data file uploads with a JSON `request` part and a `file` part. The `file` part must be a `readable stream` that contains a file in a supported format: GIF, JPEG, PNG, TIFF, BMP, or PDF.  Invoices can have up to 10 attachments with a total file size of 25 MB. Attachments can be added only to invoices in the `DRAFT`, `SCHEDULED`, `UNPAID`, or `PARTIALLY_PAID` state.  __NOTE:__ When testing in the Sandbox environment, the total file size is limited to 1 KB.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the [invoice](entity:Invoice) to attach the file to. | [required] |
**request** | Option<[**models::CreateInvoiceAttachmentRequest**](CreateInvoiceAttachmentRequest.md)> |  |  |
**image_file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::CreateInvoiceAttachmentResponse**](CreateInvoiceAttachmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_invoice

> models::DeleteInvoiceResponse delete_invoice(invoice_id, version)
DeleteInvoice

Deletes the specified invoice. When an invoice is deleted, the  associated order status changes to CANCELED. You can only delete a draft  invoice (you cannot delete a published invoice, including one that is scheduled for processing).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the invoice to delete. | [required] |
**version** | Option<**i32**> | The version of the [invoice](entity:Invoice) to delete. If you do not know the version, you can call [GetInvoice](api-endpoint:Invoices-GetInvoice) or  [ListInvoices](api-endpoint:Invoices-ListInvoices). |  |

### Return type

[**models::DeleteInvoiceResponse**](DeleteInvoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_invoice_attachment

> models::DeleteInvoiceAttachmentResponse delete_invoice_attachment(invoice_id, attachment_id)
DeleteInvoiceAttachment

Removes an attachment from an invoice and permanently deletes the file. Attachments can be removed only from invoices in the `DRAFT`, `SCHEDULED`, `UNPAID`, or `PARTIALLY_PAID` state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the [invoice](entity:Invoice) to delete the attachment from. | [required] |
**attachment_id** | **String** | The ID of the [attachment](entity:InvoiceAttachment) to delete. | [required] |

### Return type

[**models::DeleteInvoiceAttachmentResponse**](DeleteInvoiceAttachmentResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invoice

> models::GetInvoiceResponse get_invoice(invoice_id)
GetInvoice

Retrieves an invoice by invoice ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the invoice to retrieve. | [required] |

### Return type

[**models::GetInvoiceResponse**](GetInvoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invoices

> models::ListInvoicesResponse list_invoices(location_id, cursor, limit)
ListInvoices

Returns a list of invoices for a given location. The response  is paginated. If truncated, the response includes a `cursor` that you     use in a subsequent request to retrieve the next set of invoices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the location for which to list invoices. | [required] |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint.  Provide this cursor to retrieve the next set of results for your original query.  For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**limit** | Option<**i32**> | The maximum number of invoices to return (200 is the maximum `limit`).  If not provided, the server uses a default limit of 100 invoices. |  |

### Return type

[**models::ListInvoicesResponse**](ListInvoicesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_invoice

> models::PublishInvoiceResponse publish_invoice(invoice_id, publish_invoice_request)
PublishInvoice

Publishes the specified draft invoice.   After an invoice is published, Square  follows up based on the invoice configuration. For example, Square  sends the invoice to the customer's email address, charges the customer's card on file, or does  nothing. Square also makes the invoice available on a Square-hosted invoice page.   The invoice `status` also changes from `DRAFT` to a status  based on the invoice configuration. For example, the status changes to `UNPAID` if  Square emails the invoice or `PARTIALLY_PAID` if Square charges a card on file for a portion of the  invoice amount.  In addition to the required `ORDERS_WRITE` and `INVOICES_WRITE` permissions, `CUSTOMERS_READ` and `PAYMENTS_WRITE` are required when publishing invoices configured for card-on-file payments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the invoice to publish. | [required] |
**publish_invoice_request** | [**PublishInvoiceRequest**](PublishInvoiceRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::PublishInvoiceResponse**](PublishInvoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_invoices

> models::SearchInvoicesResponse search_invoices(search_invoices_request)
SearchInvoices

Searches for invoices from a location specified in  the filter. You can optionally specify customers in the filter for whom to  retrieve invoices. In the current implementation, you can only specify one location and  optionally one customer.  The response is paginated. If truncated, the response includes a `cursor`  that you use in a subsequent request to retrieve the next set of invoices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_invoices_request** | [**SearchInvoicesRequest**](SearchInvoicesRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::SearchInvoicesResponse**](SearchInvoicesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_invoice

> models::UpdateInvoiceResponse update_invoice(invoice_id, update_invoice_request)
UpdateInvoice

Updates an invoice. This endpoint supports sparse updates, so you only need to specify the fields you want to change along with the required `version` field. Some restrictions apply to updating invoices. For example, you cannot change the `order_id` or `location_id` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The ID of the invoice to update. | [required] |
**update_invoice_request** | [**UpdateInvoiceRequest**](UpdateInvoiceRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::UpdateInvoiceResponse**](UpdateInvoiceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

