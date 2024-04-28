export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH"

--[=[
	@interface ServeRequest
	@within Net

	Data type for requests in `net.serve`.

	This is a dictionary containing the following values:

	* `path` - The path being requested, relative to the root. Will be `/` if not specified
	* `query` - A table of key-value pairs representing query parameters in the request path
	* `method` - The HTTP method verb, such as `"GET"`, `"POST"`, `"PATCH"`, `"PUT"`, or `"DELETE"`. Will always be uppercase
	* `headers` - A table of key-value pairs representing headers
	* `body` - The request body, or an empty string if one was not given
]=]
export type ServeRequest = {
	path: string,
	query: { [string]: string? },
	method: HttpMethod,
	headers: { [string]: string },
	body: string,
}

--[=[
	@interface ServeResponse
	@within Net

	Response type for requests in `net.serve`.

	This is a dictionary that may contain one or more of the following values:

	* `status` - The status code for the request, in the range `100` -> `599`
	* `headers` - A table of key-value pairs representing headers
	* `body` - The response body
]=]
export type ServeResponse = {
	status: number?,
	headers: { [string]: string }?,
	body: (string | buffer)?,
}

export type ServeHttpHandler = (request: ServeRequest) -> string | ServeResponse

return {}
