# https://adventofcode.com/2021/day/15

using DataStructures


function load_map(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  cavemap = Matrix{Int}(undef, length(lines), length(lines[1]))

  for i in 1:length(lines)
    cavemap[i, :] = map(x -> parse(Int, x), split(lines[i], ""))
  end

  cavemap
end

function show_map(cavemap)
  for row in eachrow(cavemap)
    println(join(row, ""))
  end

  println("")
end

function get_neighbors(vertex, dims)
  row, col = vertex
  max_row, max_col = dims

  neighbors = filter(
    x -> x[1] >= 1 && x[1] <= max_row && x[2] >= 1 && x[2] <= max_col,
    [
      (row - 1, col),
      (row, col - 1),
      (row, col + 1),
      (row + 1, col),
    ]
  )
end

function smallest_distance(queue, distances)
  s_distance = Inf
  s_vertex = nothing

  for vertex in queue
    if distances[vertex] < s_distance
      s_vertex = vertex
      s_distance = distances[vertex]
    end
  end

  s_vertex
end

# https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
function shortest_path(cavemap, start, finish)
  queue = PriorityQueue()
  distances = Dict()
  previous = Dict()

  for r in 1:size(cavemap, 1)
    for c in 1:size(cavemap, 2)
      vertex = (r, c)
      enqueue!(queue, vertex, Inf)
      distances[vertex] = Inf
      previous[vertex] = nothing
    end
  end

  distances[start] = 0

  while !isempty(queue)
    u = dequeue!(queue)

    if u == finish
      break
    end

    for neighbor in get_neighbors(u, size(cavemap))
      if haskey(queue, neighbor)
        alt = distances[u] + cavemap[neighbor[1], neighbor[2]]
        if alt < distances[neighbor]
          distances[neighbor] = alt
          previous[neighbor] = u
          queue[neighbor] = alt
        end
      end
    end
  end

  path = []
  u = finish
  if previous[u] != nothing || finish == start
    while u != nothing
      push!(path, u)
      u = previous[u]
    end
  end

  reverse(path)
end

function lowest_risk(cavemap)
  path = shortest_path(cavemap, (1, 1), size(cavemap))
  sum(
    map(
      x -> cavemap[x[1], x[2]],
      path[2:length(path)]
    )
  )
end

function boost_map(cavemap, boost)
  height, width = size(cavemap)
  new_map = fill(0, height, width)

  for r in 1:height
    for c in 1:width
      new_map[r, c] = cavemap[r, c] + boost
      while new_map[r, c] > 9
        new_map[r, c] -= 9
      end
    end
  end

  new_map
end

function expand_map(cavemap)
  new_map = copy(cavemap)

  for i in 1:4
    new_map = cat(new_map, boost_map(cavemap, i), dims=2)
  end

  row = copy(new_map)
  for i in 1:4
    new_map = cat(new_map, boost_map(row, i), dims=1)
  end
  
  new_map
end


cavemap = load_map(ARGS[1])
full_cavemap = expand_map(cavemap)

# show_map(cavemap)
# show_map(full_cavemap)

println("Part 1: ", lowest_risk(cavemap))
println("Part 2: ", lowest_risk(full_cavemap))
