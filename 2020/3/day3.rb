# https://adventofcode.com/2020/day/3

tree_map = File
  .new(ARGV[0])
  .readlines
  .map { |line| line.chomp.split('').map { |x| x == '#' } }


def count_trees(tree_map, right, down)
  trees = 0
  row = 0
  col = 0

  while row < tree_map.count
    trees += 1 if tree_map[row][col]

    col = (col + right) % tree_map[0].count
    row += down
  end

  trees
end

slopes = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
]

result = slopes
  .map do |right, down|
    trees = count_trees(tree_map, right, down)
    puts "Slope #{right}/#{down}: #{trees}"
    trees
  end
  .reduce(:*)

puts "Total Product: #{result}"
