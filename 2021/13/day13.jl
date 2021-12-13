# https://adventofcode.com/2021/day/13


function load_page(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  dots = []
  folds = []
  for line in lines
    if length(line) == 0
      continue
    end

    if line[1] == 'f'
      instruction, position = split(line, '=')
      push!(folds, [instruction[length(instruction)], parse(Int, position)])
    else
      push!(dots, map(x -> parse(Int, x), split(line, ',')))
    end
  end

  dots, folds
end

function make_grid(dots)
  max_x = maximum(identity, map(d -> d[1], dots))
  max_y = maximum(identity, map(d -> d[2], dots))

  grid = zeros(Int, max_y + 1, max_x + 1)

  for dot in dots
    grid[dot[2] + 1, dot[1] + 1] += 1
  end

  grid
end

function fold_grid(grid, axis, position)
  if axis == 'y'
    folded = zeros(Int, position, size(grid, 2))
    line_a = position
    line_b = position + 2

    while line_a >= 1 && line_b <= size(grid, 1)
      for col in 1:size(folded, 2)
        folded[line_a, col] = grid[line_a, col] + grid[line_b, col]
      end

      line_a -= 1
      line_b += 1
    end

  elseif axis == 'x'
    folded = zeros(Int, size(grid, 1), position)
    line_a = position
    line_b = position + 2

    while line_a >= 1 && line_b <= size(grid, 2)
      for col in 1:size(folded, 1)
        folded[col, line_a] = grid[col, line_a] + grid[col, line_b]
      end

      line_a -= 1
      line_b += 1
    end
  end

  folded
end

function multiple_fold_grid(grid, folds)
  for fold in folds
    grid = fold_grid(grid, fold[1], fold[2])
  end

  grid
end

function show_grid(grid)
  for row in eachrow(grid)
    println(join(map(x -> x > 0 ? '#' : '.', row), ""))
  end

  println("")
end

function count_dots(grid)
  reduce(
    +,
    map(
      row -> length(filter(v -> v > 0, row)),
      eachrow(grid)
    )
  )
end


dots, folds = load_page(ARGS[1])
grid = make_grid(dots)
# show_grid(grid)

println("Part 1: ", count_dots(fold_grid(grid, folds[1][1], folds[1][2])))

println("Part 2:")
show_grid(multiple_fold_grid(grid, folds))
