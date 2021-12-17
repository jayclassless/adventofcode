# https://adventofcode.com/2021/day/17


function load_target(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  xdesc, ydesc = split(lines[1][14:length(lines[1])], ", ")
  xl, xu = split(xdesc[3:length(xdesc)], "..")
  yl, yu = split(ydesc[3:length(ydesc)], "..")

  return (
    (parse(Int, xl), parse(Int, xu)),
    (parse(Int, yl), parse(Int, yu))
  )
end

function step(position, velocity)
  x = position[1] + velocity[1]
  y = position[2] + velocity[2]

  dx = velocity[1]
  if dx > 0
    dx -= 1
  elseif dx < 0
    dx += 1
  end
  dy = velocity[2] - 1

  (
    (x, y), # new position
    (dx, dy), # updated velocity
  )
end

function is_hit(position, target)
  position[1] >= target[1][1] &&
    position[1] <= target[1][2] &&
    position[2] >= target[2][1] &&
    position[2] <= target[2][2]
end

function is_beyond(position, target)
  position[1] > target[1][2] ||
    position[2] < target[2][1]
end

function chart_path(velocity, target)
  path = []
  position = (0, 0)

  while !is_beyond(position, target)
    push!(path, position)
    if is_hit(position, target)
      return path
    end

    position, velocity = step(position, velocity)
  end

  nothing
end

function find_successful_velocities(target)
  velocity = (1,1)
  hits = Dict()

  min_dx = 1 # has to at least be moving forward
  max_dx = target[1][2] # can't be past the target on the first shot
  min_dy = target[2][1] # can't be below the target on the first shot 
  max_dy = 999 # arbitrary big number that seems to work for these inputs :(

  for dx in min_dx:max_dx
    for dy in min_dy:max_dy
      path = chart_path((dx, dy), target)
      if path != nothing
        hits[(dx, dy)] = maximum(map(x -> x[2], path))
      end
    end
  end

  hits
end

function part1(target)
  maximum(values(find_successful_velocities(target)))
end

function part2(target)
  length(find_successful_velocities(target))
end


target = load_target(ARGS[1])
println("Target Area: ", target)
println("Part 1: ", part1(target))
println("Part 2: ", part2(target))
