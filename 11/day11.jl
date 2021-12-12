# https://adventofcode.com/2021/day/11


function load_map(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  octomap = Matrix{Int}(undef, length(lines), length(lines[1]))

  for i in 1:length(lines)
    octomap[i, :] = map(x -> parse(Int, x), split(lines[i], ""))
  end

  octomap
end

function show_map(octomap)
  for row in eachrow(octomap)
    println(join(row, ""))
  end
  println("")
end

function flash!(octomap, row, col)
  neighbors = filter(
    x -> x[1] >= 1 && x[1] <= 10 && x[2] >= 1 && x[2] <= 10,
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
  )

  for (row, col) in neighbors
    octomap[row, col] += 1
  end
end

function run_step(octomap)
  omap = copy(octomap)

  # charge each position
  for row in 1:size(omap, 1)
    for col in 1:size(omap, 2)
      omap[row, col] += 1
    end
  end

  # process flashes
  flashed = []
  do_a_loop = true
  while do_a_loop
    do_a_loop = false
    for row in 1:size(omap, 1)
      for col in 1:size(omap, 2)
        if omap[row, col] > 9 && findfirst(x -> x == [row, col], flashed) == nothing
          flash!(omap, row, col)
          push!(flashed, [row, col])
          do_a_loop = true
        end
      end
    end
  end

  # reset those that flashes
  for row in 1:size(omap, 1)
    for col in 1:size(omap, 2)
      if omap[row, col] > 9
        omap[row, col] = 0
      end
    end
  end

  [omap, length(flashed)]
end

function part1(octomap)
  total_flashes = 0

  for i in 1:100
    octomap, flashes = run_step(octomap)
    total_flashes += flashes
  end

  total_flashes
end

function part2(octomap)
  flashes = 0
  step = 0

  while flashes != 100
    step += 1
    octomap, flashes = run_step(octomap)
  end

  step
end


octomap = load_map(ARGS[1])
show_map(octomap)

println("Part 1: ", part1(octomap))
println("Part 2: ", part2(octomap))
