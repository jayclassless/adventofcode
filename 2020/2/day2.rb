# https://adventofcode.com/2020/day/2

lines = File
  .new(ARGV[0])
  .readlines
  .map { |line| line.split(':').map(&:strip) }


valid_passwords = lines
  .filter do |rule, password|
    range, char = rule.split(' ')
    lower, upper = range.split('-').map { |x| Integer(x) }

    password.count(char) >= lower && password.count(char) <= upper
  end
  .count

puts "Part 1: #{valid_passwords}"


valid_passwords = lines
  .filter do |rule, password|
    range, char = rule.split(' ')
    lower, upper = range.split('-').map { |x| Integer(x) }

    (password[lower - 1] == char) ^ (password[upper - 1] == char)
  end
  .count

puts "Part 2: #{valid_passwords}"
