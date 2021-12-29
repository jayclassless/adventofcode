# https://adventofcode.com/2021/day/25


function load_map(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  m = Matrix{String}(undef, length(lines), length(lines[1]))

  for i in 1:length(lines)
    m[i, :] = split(lines[i], "")
  end

  m
end

function show_map(m)
  for row in eachrow(m)
    println(join(row, ""))
  end
  println("")
end

function do_east_step(m)
  height, width = size(m)
  updated = copy(m)

  for row in 1:height
    for col in 1:width
      if m[row, col] == ">"
        new_col = col + 1
        if new_col > width
          new_col = 1
        end
        if m[row, new_col] == "."
          updated[row, new_col] =  ">"
          updated[row, col] = "."
        end
      end
    end
  end

  updated
end

function do_south_step(m)
  height, width = size(m)
  updated = copy(m)

  for row in 1:height
    for col in 1:width
      if m[row, col] == "v"
        new_row = row + 1
        if new_row > height
          new_row = 1
        end
        if m[new_row, col] == "."
          updated[new_row, col] =  "v"
          updated[row, col] = "."
        end
      end
    end
  end

  updated
end

function do_step(m)
  do_south_step(do_east_step(m))
end

function part1(m)
  step = 0
  prev = nothing
  updated = m

  while prev != updated
    prev = updated
    step += 1
    updated = do_step(prev)
  end

  step
end


floor_map = load_map(ARGS[1])
# show_map(floor_map)
println("Part 1: ", part1(floor_map))
