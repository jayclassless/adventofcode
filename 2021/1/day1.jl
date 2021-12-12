# https://adventofcode.com/2021/day/1

function get_measurements(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)
 
  map(x -> parse(Int32, x), lines)
end

function count_increases(values)
  differences = map(
    x -> x[1] - x[2],
    zip(
      values[1:(length(values) - 1)],
      values[2:length(values)]
    )
  )

  length(filter(x -> x < 0, differences))
end

function windowed(values, size)
  combined = Int32[]

  for i in 1:(length(values) - (size - 1))
    push!(combined, sum(values[i:(i + (size - 1))]))
  end

  combined
end


measurements = get_measurements(ARGS[1])
println("Simple Increases: ", count_increases(measurements))
println("Windowed Increases: ", count_increases(windowed(measurements, 3)))
