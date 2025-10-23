-- String Utilities Module
-- Provides string manipulation and analysis functions
-- Used by package-app example

local function validate_string(value, name)
    if type(value) ~= "string" then
        error(name .. " must be a string, got " .. type(value))
    end
end

module.exports = {
    version = "1.0.0",
    
    uppercase = function(str)
        validate_string(str, "str")
        return str:upper()
    end,
    
    lowercase = function(str)
        validate_string(str, "str")
        return str:lower()
    end,
    
    capitalize = function(str)
        validate_string(str, "str")
        if #str == 0 then
            return str
        end
        return str:sub(1, 1):upper() .. str:sub(2):lower()
    end,
    
    reverse = function(str)
        validate_string(str, "str")
        return str:reverse()
    end,
    
    length = function(str)
        validate_string(str, "str")
        return #str
    end,
    
    trim = function(str)
        validate_string(str, "str")
        return str:match("^%s*(.-)%s*$")
    end,
    
    ltrim = function(str)
        validate_string(str, "str")
        return str:match("^%s*(.*)$")
    end,
    
    rtrim = function(str)
        validate_string(str, "str")
        return str:match("^(.-)%s*$")
    end,
    
    contains = function(str, substring)
        validate_string(str, "str")
        validate_string(substring, "substring")
        return str:find(substring, 1, true) ~= nil
    end,
    
    starts_with = function(str, prefix)
        validate_string(str, "str")
        validate_string(prefix, "prefix")
        return str:sub(1, #prefix) == prefix
    end,
    
    ends_with = function(str, suffix)
        validate_string(str, "str")
        validate_string(suffix, "suffix")
        if #suffix > #str then
            return false
        end
        return str:sub(-#suffix) == suffix
    end,
    
    replicate = function(str, count)
        validate_string(str, "str")
        if type(count) ~= "number" or count < 0 then
            error("count must be a non-negative number")
        end
        local result = ""
        for _ = 1, count do
            result = result .. str
        end
        return result
    end,
    
    split = function(str, delimiter)
        validate_string(str, "str")
        validate_string(delimiter, "delimiter")
        local result = {}
        
        if delimiter == "" then
            for i = 1, #str do
                table.insert(result, str:sub(i, i))
            end
            return result
        end
        
        local start = 1
        while true do
            local found = str:find(delimiter, start, true)
            if not found then
                table.insert(result, str:sub(start))
                break
            end
            table.insert(result, str:sub(start, found - 1))
            start = found + #delimiter
        end
        
        return result
    end,
    
    join = function(strings, delimiter)
        if type(strings) ~= "table" then
            error("strings must be a table")
        end
        validate_string(delimiter, "delimiter")
        return table.concat(strings, delimiter)
    end,
    
    replace = function(str, old, new)
        validate_string(str, "str")
        validate_string(old, "old")
        validate_string(new, "new")
        
        if old == "" then
            error("old string cannot be empty")
        end
        
        return str:gsub(old:gsub("[%(%)%.%+%-%*%?%[%]%^%$%%]", "%%%0"), new, 1)
    end,
    
    replace_all = function(str, old, new)
        validate_string(str, "str")
        validate_string(old, "old")
        validate_string(new, "new")
        
        if old == "" then
            error("old string cannot be empty")
        end
        
        return str:gsub(old:gsub("[%(%)%.%+%-%*%?%[%]%^%$%%]", "%%%0"), new)
    end,
    
    word_count = function(str)
        validate_string(str, "str")
        local count = 0
        for _ in str:gmatch("%S+") do
            count = count + 1
        end
        return count
    end,
    
    char_frequency = function(str)
        validate_string(str, "str")
        local freq = {}
        for i = 1, #str do
            local char = str:sub(i, i)
            freq[char] = (freq[char] or 0) + 1
        end
        return freq
    end,
}
