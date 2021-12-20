# https://adventofcode.com/2021/day/20


function load_data(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  pixelmap = lines[1]

  image = Matrix{String}(undef, (length(lines) - 2), length(lines[3]))

  for i in 3:length(lines)
    image[i - 2, :] = split(lines[i], "")
  end

  pixelmap, image
end

function get_sample(image, row, col, background)
  max_row, max_col = size(image)

  targets = [
    (row - 1, col - 1),
    (row - 1, col),
    (row - 1, col + 1),
    (row, col - 1),
    (row, col),
    (row, col + 1),
    (row + 1, col - 1),
    (row + 1, col),
    (row + 1, col + 1),
  ]

  map(
    x -> (x[1] < 1 || x[1] > max_row || x[2] < 1 || x[2] > max_col) ? background : image[x[1], x[2]],
    targets
  )
end

function sample_to_int(sample)
  binary = join(map(x -> (x == ".") ? "0" : "1", sample), "")
  parse(Int, binary, base=2)
end

function refine_pixel(image, row, col, pixelmap, background)
  sample = get_sample(image, row, col, background)
  string(pixelmap[sample_to_int(sample) + 1])
end

function refine_image(image, pixelmap, iterations)
  new_image = nothing

  for iter in 1:iterations
    new_image = Matrix{String}(undef, size(image, 1) + 2, size(image, 2) + 2)

    for row in 1:size(new_image, 1)
      for col in 1:size(new_image, 2)
        background = "."
        if iseven(iter) && pixelmap[1] == '#'
          background = "#"
        end

        new_image[row, col] = refine_pixel(image, row - 1, col - 1, pixelmap, background)
      end
    end

    image = new_image
  end

  new_image
end

function show_image(image)
  for row in eachrow(image)
    println(join(row, ""))
  end
  println("")
end

function count_lit(image)
  lit = 0

  for row in eachrow(image)
    for pixel in row
      if pixel == "#"
        lit += 1
      end
    end
  end

  lit
end

function part1(image, pixelmap)
  new_image = refine_image(image, pixelmap, 2)
  count_lit(new_image)
end

function part2(image, pixelmap)
  new_image = refine_image(image, pixelmap, 50)
  count_lit(new_image)
end


pixelmap, image = load_data(ARGS[1])
println("Part 1: ", part1(image, pixelmap))
println("Part 2: ", part2(image, pixelmap))
