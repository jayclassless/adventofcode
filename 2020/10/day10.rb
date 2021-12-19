# https://adventofcode.com/2020/day/10

adapters = File
  .new(ARGV[0])
  .readlines
  .map { |line| Integer(line) }


def count_increments(adapters)
  increments = {
    3 => 1,
  }

  last = 0
  adapters.sort.each do |adapter|
    diff = adapter - last

    increments[diff] = 0 unless increments.key?(diff)
    increments[diff] += 1

    last = adapter
  end

  puts increments
  increments
end

def part1(adapters)
  increments = count_increments(adapters)
  increments[1] * increments[3]
end

def possible_solutions(adapters, i=0, cache=nil)
  return 1 if i == (adapters.length - 1)

  cache = {} if cache.nil?
  return cache[i] if cache.key?(i)

  solutions = 0
  for j in (i + 1)..[(i + 3), (adapters.length - 1)].min
    solutions += possible_solutions(adapters, j, cache) if (adapters[j] - adapters[i]) <= 3
  end

  cache[i] = solutions

  solutions
end

def part2(adapters)
  possible_solutions([0] + adapters.sort + [adapters.max + 3])
end

puts "Part 1: #{part1(adapters)}"
puts "Part 2: #{part2(adapters)}"
