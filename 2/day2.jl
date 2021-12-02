# https://adventofcode.com/2021/day/2

using DelimitedFiles


function get_commands(filename)
  commands = readdlm(filename, ' ', String, '\n')
  map(x -> [x[1], parse(Int32, x[2])], eachrow(commands))
end

commands = get_commands(ARGS[1])

function part1_position(commands)
  horizontal = 0
  depth = 0

  for cmd in commands
    action = cmd[1]
    distance = cmd[2]

    if action == "forward"
      horizontal += distance
    elseif action == "up"
      depth -= distance
    elseif action == "down"
      depth += distance
    end
  end

  [horizontal, depth]
end

function part2_position(commands)
  horizontal = 0
  depth = 0
  aim = 0

  for cmd in commands
    action = cmd[1]
    distance = cmd[2]

    if action == "forward"
      horizontal += distance
      depth += aim * distance
    elseif action == "up"
      aim -= distance
    elseif action == "down"
      aim += distance
    end
  end

  [horizontal, depth]
end


println("Part 1")
horizontal, depth = part1_position(commands)
println("Final Horizontal Position: ", horizontal)
println("Final Depth: ", depth)
println("Combined Position: ", horizontal * depth)

println("\nPart 2")
horizontal, depth = part2_position(commands)
println("Final Horizontal Position: ", horizontal)
println("Final Depth: ", depth)
println("Combined Position: ", horizontal * depth)
