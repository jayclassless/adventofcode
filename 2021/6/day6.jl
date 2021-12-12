# https://adventofcode.com/2021/day/6


function load_ages(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  map(x -> parse(Int, x), split(lines[1], ','))
end

function bucketize_ages(ages)
  buckets = zeros(Int, 9)

  for age in ages
    buckets[age + 1] += 1
  end

  buckets
end

function simulate_day(buckets)
  new_buckets = zeros(Int, 9)

  new_buckets[7] += buckets[1] # spawner reset
  new_buckets[9] += buckets[1] # newborns

  for i in 2:9
    new_buckets[i - 1] += buckets[i] # getting old
  end

  new_buckets
end

function simulate(ages, days)
  buckets = bucketize_ages(ages)

  for i in 1:days
    buckets = simulate_day(buckets)
  end

  buckets
end

function count_fish(buckets)
  reduce(+, buckets)
end


ages = load_ages(ARGS[1])
buckets = bucketize_ages(ages)

println("Starting fish: ", length(ages))
println("Fish after 18 days: ", count_fish(simulate(ages, 18)))
println("Fish after 80 days: ", count_fish(simulate(ages, 80)))
println("Fish after 256 days: ", count_fish(simulate(ages, 256)))
