# https://adventofcode.com/2021/day/10


function load_navigation(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  lines
end

CHUNKS = [
  ['[', ']'],
  ['(', ')'],
  ['{', '}'],
  ['<', '>'],
]

function is_open(token)
  findfirst(
    x -> x == token,
    map(x -> x[1], CHUNKS)
  ) != nothing
end

function is_pair(opener, closer)
  findfirst(
    x -> x == [opener, closer],
    CHUNKS
  ) != nothing
end

function get_closer(opener)
  first(filter(x -> x[1] == opener, CHUNKS))[2]
end

function find_corruption(line)
  opened = []

  for token in line
    if is_open(token)
      push!(opened, token)
    else
      last_opened = pop!(opened)
      if !is_pair(last_opened, token)
        return token
      end
    end
  end

  nothing
end

PART1_SCORES = Dict(
  ')' => 3,
  ']' => 57,
  '}' => 1197,
  '>' => 25137,
)

function part1(navigation)
  score = 0

  for line in navigation
    result = find_corruption(line)

    if result != nothing
      score += PART1_SCORES[result]
    end
  end

  score
end

function find_completion(line)
  opened = []

  for token in line
    if is_open(token)
      push!(opened, token)
    else
      pop!(opened)
    end
  end

  if length(opened) > 0
    return join(reverse(map(x -> get_closer(x), opened)), "")
  else
    return nothing
  end
end

PART2_SCORES = Dict(
  ')' => 1,
  ']' => 2,
  '}' => 3,
  '>' => 4,
)

function part2(navigation)
  scores = []

  for line in navigation
    if find_corruption(line) == nothing
      completion = find_completion(line)
      if completion != nothing
        score = 0

        for token in completion
          score *= 5
          score += PART2_SCORES[token]
        end

        push!(scores, score)
      end
    end
  end

  sort!(scores)
  mid = Int(floor(length(scores) / 2) + 1)
  scores[mid]
end


navigation = load_navigation(ARGS[1])
println("Part 1: ", part1(navigation))
println("Part 2: ", part2(navigation))
