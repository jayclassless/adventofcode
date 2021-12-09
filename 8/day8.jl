# https://adventofcode.com/2021/day/8


function load_signals(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  signals = Matrix{Vector{String}}(undef, length(lines), 10)
  displays = Matrix{Vector{String}}(undef, length(lines), 4)

  for i in 1:length(lines)
    sig, disp = split(lines[i], '|')
    signals[i, :] = map(x -> split(x, ""), split(strip(sig)))
    displays[i, :] = map(x -> split(x, ""), split(strip(disp)))
  end

  [signals, displays]
end

function part1_solution(displays)
  digit_lengths = zeros(Int, 7)

  for display in eachrow(displays)
    for digit in display
      digit_lengths[length(digit)] += 1
    end
  end

  # 1 + 4 + 7 + 8
  digit_lengths[2] + digit_lengths[4] + digit_lengths[3] + digit_lengths[7]
end

function segment_mapping(signal)
  by_length = map(x -> Set{String}[], 1:7)
  for sig in signal
    push!(by_length[length(sig)], Set(sig))
  end

  # easy to identify
  digits = Dict(
    1 => by_length[2][1],
    4 => by_length[4][1],
    7 => by_length[3][1],
    8 => by_length[7][1],
  )
  segments = Dict(
    "a" => first(setdiff(digits[7], digits[1])),
  )

  # six is the only 6-len digit that does not overlap completely with seven
  digits[6] = filter(x -> !isempty(setdiff(digits[7], x)), by_length[6])[1]
  segments["c"] = first(setdiff(digits[7], digits[6]))
  segments["f"] = first(setdiff(digits[7], [segments["a"], segments["c"]]))

  # nine is the only 6-len digit that overlaps completely with four
  digits[9] = filter(x -> isempty(setdiff(digits[4], x)), by_length[6])[1]
  
  # zero is the last remaining 6-len
  digits[0] = first(setdiff(by_length[6], [digits[6], digits[9]]))

  # can find the rest of the segments with the digits we have
  segments["d"] = first(setdiff(digits[8], digits[0]))
  segments["b"] = first(setdiff(digits[4], [segments["c"], segments["d"], segments["f"]]))
  segments["g"] = first(setdiff(digits[9], [segments["a"], segments["b"], segments["c"], segments["d"], segments["f"]]))
  segments["e"] = first(setdiff(digits[8], [segments["a"], segments["b"], segments["c"], segments["d"], segments["f"], segments["g"]]))

  mapping = Dict()
  for (key, val) in segments
    mapping[val] = key
  end

  mapping
end

DIGITS = Dict(
  "abcefg" => "0",
  "cf" => "1",
  "acdeg" => "2",
  "acdfg" => "3",
  "bcdf" => "4",
  "abdfg" => "5",
  "abdefg" => "6",
  "acf" => "7",
  "abcdefg" => "8",
  "abcdfg" => "9",
)

function get_digit(mapping, display)
  translated = map(x -> mapping[x], display)
  DIGITS[join(sort(translated))]
end

function part2_solution(signals, displays)
  values = Int[]

  for i in 1:size(signals, 1)
    mapping = segment_mapping(signals[i, :])
    digits = map(x -> get_digit(mapping, x), displays[i, :])
    push!(values, parse(Int, join(digits)))
  end

  sum(values)
end


signals, displays = load_signals(ARGS[1])
println("Part 1: ", part1_solution(displays))
println("Part 2: ", part2_solution(signals, displays))
