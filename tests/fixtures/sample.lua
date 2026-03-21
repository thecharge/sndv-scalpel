local util = require("util")

local function calculate_total(items)
  local sum = 0
  for _, item in ipairs(items) do
    sum = sum + item
  end
  return sum
end

function M.run()
  return calculate_total({1, 2, 3})
end
