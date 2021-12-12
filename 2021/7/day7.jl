# https://adventofcode.com/2021/day/7


function load_positions(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  map(x -> parse(Int, x), split(lines[1], ','))
end

function bucketize_positions(positions)
  m, i = findmax(positions)
  buckets = zeros(Int, m + 1)

  for pos in positions
    buckets[pos + 1] += 1
  end

  buckets
end

function find_cheapest_solution(buckets, coster)
  costs = zeros(Int, length(buckets))

  for pos in 1:length(buckets)
    for i in 1:length(buckets)
      costs[pos] += buckets[i] * coster(abs(pos - i))
    end
  end

  findmin(costs)
end

function part1_solution(buckets)
  find_cheapest_solution(buckets, x -> x)
end

function part2_solution(buckets)
  find_cheapest_solution(buckets, x -> sum(1:x))
end


positions = load_positions(ARGS[1])
buckets = bucketize_positions(positions)

println("Part 1")
cost, position = part1_solution(buckets)
println("Cheapest position is ", position - 1, " at cost of ", cost)

println("\nPart 2")
cost, position = part2_solution(buckets)
println("Cheapest position is ", position - 1, " at cost of ", cost)
