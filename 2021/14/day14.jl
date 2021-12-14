# https://adventofcode.com/2021/day/14


function load_data(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  polymer = lines[1]
  counts = count_elements(polymer)
  polymer = count_elements(zip(polymer, polymer[2:length(polymer)]))

  insertions = Dict{String, Char}()
  for line in lines[3:length(lines)]
    pair, new_char = split(line, " -> ")
    insertions[pair] = new_char[1]
  end

  polymer, insertions, counts
end

function count_elements(value)
  counts = Dict()

  for c in value
      counts[c] = get(counts, c, 0) + 1
  end

  counts
end

function process_insertions(polymer, insertions, counts)
  new_polymer = Dict()
  new_counts = copy(counts)

  for (a, b) in keys(polymer)
    pair = join([a, b], "")
    if haskey(insertions, pair)
      new_char = insertions[pair]
      existing = polymer[(a, b)]

      new_polymer[(new_char, b)] = get(new_polymer, (new_char, b), 0) + existing
      new_polymer[(a, new_char)] = get(new_polymer, (a, new_char), 0) + existing
      new_counts[new_char] = get(new_counts, new_char, 0) + existing
    end
  end

  new_polymer, new_counts
end

function process_n_insertions(polymer, insertions, counts, n)
  new_polymer = polymer
  new_counts = counts

  for i in 1:n
    new_polymer, new_counts = process_insertions(new_polymer, insertions, new_counts)
  end

  new_polymer, new_counts
end

function score(polymer, insertions, counts, iterations)
  new_polymer, new_counts = process_n_insertions(polymer, insertions, counts, iterations)
  maximum(values(new_counts)) - minimum(values(new_counts))
end


polymer, insertions, counts = load_data(ARGS[1])
println("Part 1: ", score(polymer, insertions, counts, 10))
println("Part 2: ", score(polymer, insertions, counts, 40))
