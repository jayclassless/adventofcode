# https://adventofcode.com/2021/day/12


function load_edges(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  map(
    x -> split(x, '-'),
    lines
  )
end

function make_graph(edges)
  graph = Dict()

  for edge in edges
    if !haskey(graph, edge[1])
      graph[edge[1]] = []
    end
    push!(graph[edge[1]], edge[2])

    if !haskey(graph, edge[2])
      graph[edge[2]] = []
    end
    push!(graph[edge[2]], edge[1])
  end

  graph
end

function get_paths(graph, can_visit, start="start", term="end", current=nothing)
  if current == nothing
    current = [start]
  end

  paths = []

  for node in graph[start]
    if node == term
      push!(paths, vcat(current, [node]))
    elseif can_visit(node, current)
      paths = vcat(paths, get_paths(graph, can_visit, node, term, vcat(current, [node])))
    end
  end

  paths
end

function is_large(node)
  uppercase(node[1]) == node[1]
end

function part1_can_visit(node, current)
  is_large(node) || findfirst(x -> x == node, current) == nothing
end

function part2_can_visit(node, current)
  if is_large(node)
    return true
  elseif node == "start"
    return false
  else
    smalls = Dict()
    seen_one_twice = false
    for small in filter(x -> !is_large(x), current)
      if !haskey(smalls, small)
        smalls[small] = 0
      end
      smalls[small] += 1

      if smalls[small] == 2
        seen_one_twice = true
      end
    end

    if seen_one_twice
      if !haskey(smalls, node)
        return true
      else
        return false
      end
    else
      return true
    end
  end
end


graph = make_graph(load_edges(ARGS[1]))
println("Part 1: ", length(get_paths(graph, part1_can_visit)))
println("Part 2: ", length(get_paths(graph, part2_can_visit)))
