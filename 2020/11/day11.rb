# https://adventofcode.com/2020/day/11

OCCUPIED = "#"
EMPTY = "L"
FLOOR = "."

seats = File
  .new(ARGV[0])
  .readlines
  .map { |line| line.chomp.split("") }


def get_immediate_neighbors(seats, row, col)
  [
    [row - 1, col - 1],
    [row - 1, col],
    [row - 1, col + 1],
    [row, col - 1],
    [row, col + 1],
    [row + 1, col - 1],
    [row + 1, col],
    [row + 1, col + 1],
  ]
    .filter { |n| n[0] >= 0 && n[0] < seats.length && n[1] >= 0 && n[1] < seats[0].length }
    .map { |n| seats[n[0]][n[1]] }
end

def update(seats, get_neighbors, nearby_threshold)
  new_seats = (1..seats.length).map { |x| (1..seats[0].length).map { nil } }

  seats.each_with_index do |row, r|
    row.each_with_index do |seat, c|
      neighbors = method(get_neighbors).call(seats, r, c)

      if seat == EMPTY && !neighbors.include?(OCCUPIED)
        new_seats[r][c] = OCCUPIED
      elsif seat == OCCUPIED && (neighbors.count { |n| n == OCCUPIED }) >= nearby_threshold
        new_seats[r][c] = EMPTY
      else
        new_seats[r][c] = seat
      end
    end
  end

  new_seats
end

def show_seats(seats)
  seats.each do |row|
    puts row.join("")
  end
  puts ""
end

def part1(seats)
  loop do
    new_seats = update(seats, :get_immediate_neighbors, 4)
    break if new_seats == seats
    seats = new_seats
  end

  seats
    .map { |row| row.count { |s| s == OCCUPIED } }
    .sum
end

def get_visible_neighbors(seats, row, col)
  neighbors = []

  for x in -1..1
    for y in -1..1
      i = 1
      while 0 <= (row + (i * y)) && (row + (i * y)) < seats.length && 0 <= (col + (i * x)) && (col + (i * x)) < seats[0].length && !(x == 0 && y == 0)
        s = seats[(row + (i * y))][(col + (i * x))]
        if s != FLOOR
          neighbors << s
          break
        end
        i += 1
      end
    end
  end

  neighbors
end

def part2(seats)
  loop do
    new_seats = update(seats, :get_visible_neighbors, 5)
    break if new_seats == seats
    seats = new_seats
  end

  seats
    .map { |row| row.count { |s| s == OCCUPIED } }
    .sum
end

puts "Part 1: #{part1(seats)}"
puts "Part 2: #{part2(seats)}"
