local function serialize(val, indent, seen)
    indent = indent or 0
    seen = seen or {}
    local spaces = string.rep("  ", indent)
    
    if type(val) == "table" then
        if seen[val] then
            return "<circular>"
        end
        seen[val] = true
        
        local is_array = true
        local count = 0
        for k, v in pairs(val) do
            count = count + 1
            if type(k) ~= "number" or k ~= count then
                is_array = false
                break
            end
        end
        
        if is_array and count > 0 then
            local parts = {}
            table.insert(parts, "[")
            for i, v in ipairs(val) do
                if i > 1 then
                    table.insert(parts, ",")
                end
                table.insert(parts, "\n" .. string.rep("  ", indent + 1))
                table.insert(parts, serialize(v, indent + 1, seen))
            end
            table.insert(parts, "\n" .. spaces .. "]")
            return table.concat(parts)
        else
            local parts = {}
            table.insert(parts, "{")
            local first = true
            for k, v in pairs(val) do
                if not first then
                    table.insert(parts, ",")
                end
                first = false
                table.insert(parts, "\n" .. string.rep("  ", indent + 1))
                table.insert(parts, string.format('"%s": ', tostring(k)))
                table.insert(parts, serialize(v, indent + 1, seen))
            end
            if not first then
                table.insert(parts, "\n" .. spaces)
            end
            table.insert(parts, "}")
            return table.concat(parts)
        end
    elseif type(val) == "string" then
        return string.format('"%s"', val:gsub('"', '\\"'))
    elseif type(val) == "number" then
        return tostring(val)
    elseif type(val) == "boolean" then
        return tostring(val)
    elseif val == nil then
        return "null"
    else
        return string.format('"%s"', tostring(val))
    end
end

module.exports = {
    pretty_json = function(data)
        return serialize(data, 0)
    end,
    
    validate_url = function(url)
        if not url or url == "" then
            return false, "URL cannot be empty"
        end
        
        if not url:match("^https?://") then
            return false, "URL must start with http:// or https://"
        end
        
        return true
    end,
    
    parse_headers = function(header_args)
        local headers = {}
        for _, arg in ipairs(header_args) do
            local key, value = arg:match("^(.+):(.+)$")
            if key and value then
                headers[key:match("^%s*(.-)%s*$")] = value:match("^%s*(.-)%s*$")
            end
        end
        return headers
    end
}
