# https://adventofcode.com/2020/day/7

require 'set'

lines = File
  .new(ARGV[0])
  .readlines

rules = {}
lines.each do |line|
  bag, description = line.split(' contain ')
  bag.chomp!(" bags")
  description.chomp!

  rules[bag] = {}

  description
    .chomp('.')
    .split(', ')
    .each do |desc|
      next if desc == 'no other bags'

      num, inner_bag = desc.split(' ', limit=2)
      inner_bag = inner_bag.chomp('s').chomp(' bag')

      rules[bag][inner_bag] = Integer(num)
    end
end


def part1(bag, rules)
  outermost = Set.new

  rules.each_pair do |outer_bag, inner_bags|
    if inner_bags.key?(bag)
      outermost << outer_bag
      outermost += part1(outer_bag, rules)
    end
  end

  outermost
end

puts "Part 1: #{part1('shiny gold', rules).length}"


def part2(bag, rules)
  total = 0

  rules[bag].each_pair do |inner_bag, num|
    total += num + (part2(inner_bag, rules) * num)
  end

  total
end

puts "Part 2: #{part2('shiny gold', rules)}"
