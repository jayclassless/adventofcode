# https://adventofcode.com/2020/day/6

require 'set'

input_data = File.new(ARGV[0]).read

part1 = input_data
  .split("\n\n")
  .map do |data|
    Set.new(data.scan(/./)).length
  end
  .sum

puts "Part 1: #{part1}"


part2 = input_data
  .split("\n\n")
  .map do |data|
    data
      .split("\n")
      .map { |d| Set.new(d.scan(/./)) }
      .reduce(:&)
      .length
  end
  .sum

puts "Part 2: #{part2}"
