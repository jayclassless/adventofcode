# https://adventofcode.com/2020/day/1

entries = File
  .new(ARGV[0])
  .readlines
  .map { |line| Integer(line) }


pair = entries
  .combination(2)
  .filter { |c| c.sum == 2020 }
  .first

puts "Part 1: #{pair.reduce(:*)}"


triplet = entries
  .combination(3)
  .filter { |c| c.sum == 2020 }
  .first

puts "Part 2: #{triplet.reduce(:*)}"
