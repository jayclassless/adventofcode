# https://adventofcode.com/2021/day/5


struct Line
  x1
  y1
  x2
  y2
end

function get_vents(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)
 
  vents = Line[]

  for line in lines
    start, finish = split(line, "->")
    x1, y1 = split(start, ',')
    x2, y2 = split(finish, ',')

    push!(
      vents,
      Line(
        parse(Int32, x1),
        parse(Int32, y1),
        parse(Int32, x2),
        parse(Int32, y2),
      )
    )
  end

  vents
end

function is_straight(line)
  line.x1 == line.x2 || line.y1 == line.y2
end

function initialize_map(vents)
  max_x = maximum(
    identity,
    map(v -> v.x1 > v.x2 ? v.x1 : v.x2, vents)
  )
  max_y = maximum(
    identity,
    map(v -> v.y1 > v.y2 ? v.y1 : v.y2, vents)
  )

  zeros(Int32, max_y + 1, max_x + 1)
end

function show_map(vmap)
  for row in eachrow(vmap)
    for col in row
      print(col == 0 ? '.' : col)
    end
    print('\n')
  end
end

function interpolate(line)
  points = []

  dx = line.x2 - line.x1
  if dx != 0
    dx = Int(dx / abs(dx))
  end
  dy = line.y2 - line.y1
  if dy != 0
    dy = Int(dy / abs(dy))
  end

  x, y = line.x1, line.y1
  while true
    push!(points, [x y])

    if x == line.x2 && y == line.y2
      break
    end

    x += dx
    y += dy
  end

  points
end

function fill_map(vmap, vents)
  for line in vents
    for point in interpolate(line)
      vmap[point[2] + 1, point[1] + 1] += 1
    end
  end

  vmap
end

function num_deep_points(vmap, depth)
  num = 0

  for row in eachrow(vmap)
    num += length(filter(x -> x >= depth, row))
  end

  num
end


vents = get_vents(ARGS[1])

println("Part 1")
part1_vmap = fill_map(
  initialize_map(vents),
  filter(v -> is_straight(v), vents)
)
#show_map(part1_vmap)
println("Num 2-line intersections: ", num_deep_points(part1_vmap, 2))

println("\nPart 2")
part2_vmap = fill_map(initialize_map(vents), vents)
#show_map(part2_vmap)
println("Num 2-line intersections: ", num_deep_points(part2_vmap, 2))
