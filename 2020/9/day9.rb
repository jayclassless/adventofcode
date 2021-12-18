# https://adventofcode.com/2020/day/9

values = File
  .new(ARGV[0])
  .readlines
  .map { |line| Integer(line) }

preamble = Integer(ARGV[1])

def find_first_invalid(values, preamble)
  for i in preamble..values.length
    sums = values[(i - preamble)..(i - 1)]
      .permutation(2)
      .filter { |pair| pair[0] != pair[1] }
      .map { |pair| pair.sum }

    return values[i] if !sums.include?(values[i])
  end
end

def part1(values, preamble)
  find_first_invalid(values, preamble)
end

def part2(values, preamble)
  invalid_value = find_first_invalid(values, preamble)

  for lower in 0..values.length
    for upper in (lower + 1)..values.length
      subset = values[lower..upper]
      return subset.min + subset.max if subset.sum == invalid_value
    end
  end
end


puts "Part 1: #{part1(values, preamble)}"
puts "Part 2: #{part2(values, preamble)}"
