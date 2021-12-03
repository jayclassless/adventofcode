# https://adventofcode.com/2021/day/3


function get_diagnostics(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)
 
  lines
end

function combine_diagnostics(diagnostics, combiner)
  combined = ""

  for i in 1:length(diagnostics[1])
    ones = count(
      x -> x == '1',
      map(x -> x[i], diagnostics)
    )
    zeros = count(
      x -> x == '0',
      map(x -> x[i], diagnostics)
    )

    combined = string(combined, combiner(zeros, ones))
  end

  combined
end

function get_gamma(diagnostics)
  combine_diagnostics(
    diagnostics,
    (x, y) -> x > y ? '0' : '1'
  )
end

function get_epsilon(diagnostics)
  combine_diagnostics(
    diagnostics,
    (x, y) -> x < y ? '0' : '1'
  )
end

function filter_and_combine_diagnostics(diagnostics, combiner)
  filtered = diagnostics

  for i in 1:length(diagnostics[1])
    digits = combine_diagnostics(filtered, combiner)

    filtered = filter(
      x -> x[i] == digits[i],
      filtered
    )

    if length(filtered) == 1
      break
    end
  end

  join(filtered[1])
end

function get_oxygen(diagnostics)
  filter_and_combine_diagnostics(
    diagnostics,
    (x, y) -> x == y ? '1' : (x > y ? '0' : '1')
  )
end

function get_co2(diagnostics)
  filter_and_combine_diagnostics(
    diagnostics,
    (x, y) -> x == y ? '0' : (x < y ? '0' : '1')
  )
end

function bin2dec(value)
  parse(Int32, value, base=2)
end


diagnostics = get_diagnostics(ARGS[1])
gamma = bin2dec(get_gamma(diagnostics))
epsilon = bin2dec(get_epsilon(diagnostics))
oxygen = bin2dec(get_oxygen(diagnostics))
co2 = bin2dec(get_co2(diagnostics))
println("Gamma Rate: ", gamma)
println("Epsilon Rate: ", epsilon)
println("Power Consumption: ", gamma * epsilon)
println("Oxygen Generator Rating: ", oxygen)
println("CO2 Scrubber Rating: ", co2)
println("Life Support Rating: ", oxygen * co2)
