--[=[
    @class HTTP
]=]

--[=[
    @type HttpMethod "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH"
    @within HTTP
]=]

--[=[
    @type HttpHandler (request: Request) -> string | Response
    @within HTTP
]=]

--[=[
    @interface Request
    @within HTTP
    .path string
    .query { [string]: string? }
    .method HttpMethod
    .headers { [string]: string }
    .body (string | buffer)
]=]

--[=[
    @interface Response
    @within HTTP
    .status number?
    .headers { [string]: string }?
    .body (string | buffer)?
]=]

return {}
