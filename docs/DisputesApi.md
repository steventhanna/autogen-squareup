# \DisputesApi

All URIs are relative to *https://connect.squareup.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_dispute**](DisputesApi.md#accept_dispute) | **POST** /v2/disputes/{dispute_id}/accept | AcceptDispute
[**create_dispute_evidence_file**](DisputesApi.md#create_dispute_evidence_file) | **POST** /v2/disputes/{dispute_id}/evidence-files | CreateDisputeEvidenceFile
[**create_dispute_evidence_text**](DisputesApi.md#create_dispute_evidence_text) | **POST** /v2/disputes/{dispute_id}/evidence-text | CreateDisputeEvidenceText
[**delete_dispute_evidence**](DisputesApi.md#delete_dispute_evidence) | **DELETE** /v2/disputes/{dispute_id}/evidence/{evidence_id} | DeleteDisputeEvidence
[**list_dispute_evidence**](DisputesApi.md#list_dispute_evidence) | **GET** /v2/disputes/{dispute_id}/evidence | ListDisputeEvidence
[**list_disputes**](DisputesApi.md#list_disputes) | **GET** /v2/disputes | ListDisputes
[**retrieve_dispute**](DisputesApi.md#retrieve_dispute) | **GET** /v2/disputes/{dispute_id} | RetrieveDispute
[**retrieve_dispute_evidence**](DisputesApi.md#retrieve_dispute_evidence) | **GET** /v2/disputes/{dispute_id}/evidence/{evidence_id} | RetrieveDisputeEvidence
[**submit_evidence**](DisputesApi.md#submit_evidence) | **POST** /v2/disputes/{dispute_id}/submit-evidence | SubmitEvidence



## accept_dispute

> models::AcceptDisputeResponse accept_dispute(dispute_id)
AcceptDispute

Accepts the loss on a dispute. Square returns the disputed amount to the cardholder and updates the dispute state to ACCEPTED.  Square debits the disputed amount from the seller’s Square account. If the Square account does not have sufficient funds, Square debits the associated bank account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute you want to accept. | [required] |

### Return type

[**models::AcceptDisputeResponse**](AcceptDisputeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dispute_evidence_file

> models::CreateDisputeEvidenceFileResponse create_dispute_evidence_file(dispute_id, request, image_file)
CreateDisputeEvidenceFile

Uploads a file to use as evidence in a dispute challenge. The endpoint accepts HTTP multipart/form-data file uploads in HEIC, HEIF, JPEG, PDF, PNG, and TIFF formats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute for which you want to upload evidence. | [required] |
**request** | Option<[**models::CreateDisputeEvidenceFileRequest**](CreateDisputeEvidenceFileRequest.md)> |  |  |
**image_file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::CreateDisputeEvidenceFileResponse**](CreateDisputeEvidenceFileResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dispute_evidence_text

> models::CreateDisputeEvidenceTextResponse create_dispute_evidence_text(dispute_id, create_dispute_evidence_text_request)
CreateDisputeEvidenceText

Uploads text to use as evidence for a dispute challenge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute for which you want to upload evidence. | [required] |
**create_dispute_evidence_text_request** | [**CreateDisputeEvidenceTextRequest**](CreateDisputeEvidenceTextRequest.md) | An object containing the fields to POST for the request.  See the corresponding object definition for field details. | [required] |

### Return type

[**models::CreateDisputeEvidenceTextResponse**](CreateDisputeEvidenceTextResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dispute_evidence

> models::DeleteDisputeEvidenceResponse delete_dispute_evidence(dispute_id, evidence_id)
DeleteDisputeEvidence

Removes specified evidence from a dispute. Square does not send the bank any evidence that is removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute from which you want to remove evidence. | [required] |
**evidence_id** | **String** | The ID of the evidence you want to remove. | [required] |

### Return type

[**models::DeleteDisputeEvidenceResponse**](DeleteDisputeEvidenceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dispute_evidence

> models::ListDisputeEvidenceResponse list_dispute_evidence(dispute_id, cursor)
ListDisputeEvidence

Returns a list of evidence associated with a dispute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute. | [required] |
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |

### Return type

[**models::ListDisputeEvidenceResponse**](ListDisputeEvidenceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_disputes

> models::ListDisputesResponse list_disputes(cursor, states, location_id)
ListDisputes

Returns a list of disputes associated with a particular account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | A pagination cursor returned by a previous call to this endpoint. Provide this cursor to retrieve the next set of results for the original query. For more information, see [Pagination](https://developer.squareup.com/docs/build-basics/common-api-patterns/pagination). |  |
**states** | Option<[**DisputeState**](.md)> | The dispute states used to filter the result. If not specified, the endpoint returns all disputes. |  |
**location_id** | Option<**String**> | The ID of the location for which to return a list of disputes. If not specified, the endpoint returns disputes associated with all locations. |  |

### Return type

[**models::ListDisputesResponse**](ListDisputesResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_dispute

> models::RetrieveDisputeResponse retrieve_dispute(dispute_id)
RetrieveDispute

Returns details about a specific dispute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute you want more details about. | [required] |

### Return type

[**models::RetrieveDisputeResponse**](RetrieveDisputeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_dispute_evidence

> models::RetrieveDisputeEvidenceResponse retrieve_dispute_evidence(dispute_id, evidence_id)
RetrieveDisputeEvidence

Returns the metadata for the evidence specified in the request URL path.  You must maintain a copy of any evidence uploaded if you want to reference it later. Evidence cannot be downloaded after you upload it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute from which you want to retrieve evidence metadata. | [required] |
**evidence_id** | **String** | The ID of the evidence to retrieve. | [required] |

### Return type

[**models::RetrieveDisputeEvidenceResponse**](RetrieveDisputeEvidenceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_evidence

> models::SubmitEvidenceResponse submit_evidence(dispute_id)
SubmitEvidence

Submits evidence to the cardholder's bank.  The evidence submitted by this endpoint includes evidence uploaded using the [CreateDisputeEvidenceFile](api-endpoint:Disputes-CreateDisputeEvidenceFile) and [CreateDisputeEvidenceText](api-endpoint:Disputes-CreateDisputeEvidenceText) endpoints and evidence automatically provided by Square, when available. Evidence cannot be removed from a dispute after submission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dispute_id** | **String** | The ID of the dispute for which you want to submit evidence. | [required] |

### Return type

[**models::SubmitEvidenceResponse**](SubmitEvidenceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

