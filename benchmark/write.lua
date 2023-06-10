wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

function random_string()
    local random = tostring(math.random(os.time()))
    return random
end

function request()
    local body = '{"Set":{"key":"' .. random_string() .. '","value":"' .. random_string() .. '"}}'
    return wrk.format(nil, nil, nil, body)
end
