# https://adventofcode.com/2021/day/18

using Combinatorics


mutable struct Digit
  value
  depth
end

function parse_number(value)
  number = Digit[]

  depth = 0
  for char in value
    if char == '['
      depth += 1
    elseif char == ']'
      depth -= 1
    elseif char != ','
      push!(number, Digit(parse(Int, char), depth))
    end
  end

  number
end

function load_numbers(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  map(x -> parse_number(x), lines)
end

function explode(number)
  for (i, (left, right)) in enumerate(zip(number, number[2:length(number)]))
    if left.depth < 5 || left.depth != right.depth
      continue
    end

    if i > 1
      number[i - 1].value += left.value
    end
    if i < (length(number) - 1)
      number[i + 2].value += right.value
    end

    return vcat(number[1:i - 1], [Digit(0, (left.depth - 1))], number[(i + 2):length(number)]), true
  end

  return number, false
end

function split(number)
  for (i, digit) in enumerate(number)
    if digit.value >= 10
      return vcat(
        number[1: i - 1],
        [Digit(Int(floor(digit.value / 2)), digit.depth + 1), Digit(Int(ceil(digit.value / 2)), digit.depth + 1)],
        number[i + 1:length(number)]
      ), true
    end
  end

  return number, false
end

function reduce(number)
  modified = true
  while modified
    number, modified = explode(number)
    if !modified
      number, modified = split(number)
    end
  end

  number
end

function add(left, right)
  reduce(vcat(
    map(x -> Digit(x.value, x.depth + 1), left),
    map(x -> Digit(x.value, x.depth + 1), right),
  ))
end

function magnitude(number)
  while length(number) > 1
    for (i, (left, right)) in enumerate(zip(number, number[2:length(number)]))
      if left.depth != right.depth
        continue
      end

      mag = (left.value * 3) + (right.value * 2)
      number = vcat(number[1:i - 1], [Digit(mag, left.depth - 1)], number[(i + 2):length(number)])
      break
    end
  end

  number[1].value
end

function part1(numbers)
  magnitude(foldl(add, numbers))
end

function part2(numbers)
  maximum(map(x -> magnitude(add(x[1], x[2])), permutations(numbers, 2)))
end


numbers = load_numbers(ARGS[1])
println("Part 1: ", part1(numbers))
println("Part 2: ", part2(numbers))
