# https://adventofcode.com/2021/day/22

using OffsetArrays

struct Step
  value
  x
  y
  z
end

function parse_step(line)
  action, coords = split(line, " ")

  value = action == "on" ? 1 : 0

  x, y, z = map(
    r -> range(r[1], r[2]),
    map(
      coord -> map(v -> parse(Int, v), split(coord[3:end], "..")),
      split(coords, ",")
    )
  )

  Step(
    value,
    x,
    y,
    z,
  )
end

function load_steps(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  map(line -> parse_step(line), lines)
end

function create_reactor(x, y, z)
  OffsetArray(
    zeros(Int8, length(x), length(y), length(z)),
    x,
    y,
    z,
  )
end

function execute_steps!(reactor, steps)
  for step in steps
    try
      reactor[step.x, step.y, step.z] .= step.value
    catch exc
      if !isa(exc, BoundsError)
        throw(exc)
      end
    end
  end
end

function count_reactor(reactor, value=1)
  length(filter(x -> x == value, reactor))
end

function part1(steps)
  reactor = create_reactor(-50:50, -50:50, -50:50)
  execute_steps!(reactor, steps)
  count_reactor(reactor)
end

function part2(steps)
  x = range(
    minimum(map(s -> s.x.start, steps)),
    maximum(map(s -> s.x.stop, steps))
  )
  y = range(
    minimum(map(s -> s.y.start, steps)),
    maximum(map(s -> s.y.stop, steps))
  )
  z = range(
    minimum(map(s -> s.z.start, steps)),
    maximum(map(s -> s.z.stop, steps))
  )

  reactor = create_reactor(x, y, z)
  execute_steps!(reactor, steps)
  count_reactor(reactor)
end


steps = load_steps(ARGS[1])
println("Part 1: ", part1(steps))
# println("Part 2: ", part2(steps))  # TODO waaaaaaaay too big for our memory
