wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

function request()
    local body = '{"Set":{"key":"foooooooooo","value":"blahblahblahblahblahblah"}}'
    return wrk.format(nil, nil, nil, body)
end
