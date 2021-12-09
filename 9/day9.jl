# https://adventofcode.com/2021/day/9


function load_heightmap(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  heightmap = Matrix{Int}(undef, length(lines), length(lines[1]))

  for i in 1:length(lines)
    heightmap[i, :] = map(x -> parse(Int, x), split(lines[i], ""))
  end

  heightmap
end

function get_neighbors(matrix, row, col)
  neighbors = []

  if row > 1
    push!(neighbors, (row - 1, col))
  end

  if row < size(matrix, 1)
    push!(neighbors, (row + 1, col))
  end

  if col > 1
    push!(neighbors, (row, col - 1))
  end

  if col < size(matrix, 2)
    push!(neighbors, (row, col + 1))
  end

  neighbors
end

function is_low_point(matrix, row, col)
  !any(x -> matrix[x[1], x[2]] <= matrix[row, col], get_neighbors(matrix, row, col))
end

function part1(heightmap)
  low_points = Int[]

  for r in 1:size(heightmap, 1)
    for c in 1:size(heightmap, 2)
      if is_low_point(heightmap, r, c)
        push!(low_points, heightmap[r, c])
      end
    end
  end

  sum(map(x -> x + 1, low_points))
end

function part2(heightmap)
  basin_sizes = Int[]

  for r in 1:size(heightmap, 1)
    for c in 1:size(heightmap, 2)
      if heightmap[r, c] == 9
        continue
      end
      points = [(r, c)]
      size = 0

      while !isempty(points)
        (i, j) = pop!(points)
        if heightmap[i, j] != 9
          size += 1
          heightmap[i, j] = 9 # we've already counted this one, exclude it later
          append!(points, get_neighbors(heightmap, i, j))
        end
      end

      push!(basin_sizes, size)
    end
  end

  sort!(basin_sizes, rev=true)
  reduce(*, basin_sizes[1:3])
end


heightmap = load_heightmap(ARGS[1])
println("Part 1: ", part1(heightmap))
println("Part 2: ", part2(heightmap))
