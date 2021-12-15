# https://adventofcode.com/2020/day/5

seats = File
  .new(ARGV[0])
  .readlines
  .map(&:chomp)


def find_row(code)
  lower = 0
  size = 128

  code.each_char do |c|
    size /= 2
    if c == "B"
      lower += size
    end
  end

  lower
end

def find_seat(code)
  lower = 0
  size = 8

  code.each_char do |c|
    size /= 2
    if c == "R"
      lower += size
    end
  end

  lower
end

def get_seat_id(code)
  row = find_row(code[0, 7])
  seat = find_seat(code[7, 3])
  (row * 8) + seat
end


seat_ids = seats
  .map { |s| get_seat_id(s) }
  .sort

puts "Part 1: #{seat_ids.last}"

seat_ids
  .each_index do |i|
    if seat_ids[i + 1] == (seat_ids[i] + 2)
      puts "Part 2: #{seat_ids[i] + 1}"
    end
  end
