# https://adventofcode.com/2021/day/21

using Memoization


mutable struct Game
  positions
  scores
  die_rolls
end


function load_game(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  positions = map(
    line -> parse(Int, split(line, ": ")[2]),
    lines
  )

  Game(
    positions,
    zeros(Int, length(positions)),
    0,
  )
end

function next_position(current, movement)
  if (movement % 10) > 0
    current += (movement % 10)
    if current > 10
      current %= 10
    end
  end
  current
end

function play_turn(game, player, die)
  game = Game(
    copy(game.positions),
    copy(game.scores),
    game.die_rolls
  )

  total_roll = sum([die() for i in 1:3])
  game.die_rolls += 3

  game.positions[player] = next_position(game.positions[player], total_roll)
  game.scores[player] += game.positions[player]

  game
end

DETERMINISTIC_DIE = 0
function die_deterministic()
  global DETERMINISTIC_DIE += 1
  if DETERMINISTIC_DIE > 100
    DETERMINISTIC_DIE %= 100
  end

  DETERMINISTIC_DIE
end

function play_game(game, die, max_score)
  player = 1

  while maximum(game.scores) < max_score
    game = play_turn(game, player, die)

    player += 1
    if player > length(game.positions)
      player %= length(game.positions)
    end
  end

  game
end

QUANTUM_ROLLS = map(sum, Iterators.product(1:3, 1:3, 1:3))
@memoize function play_game_quantum(position_a, score_a, position_b, score_b)
  if score_a >= 21
    return 1, 0
  end
  if score_b >= 21
    return 0, 1
  end

  a_wins = b_wins = 0

  for roll in QUANTUM_ROLLS
    new_position = next_position(position_a, roll)
    new_score = score_a + new_position

    b, a = play_game_quantum(position_b, score_b, new_position, new_score)
    a_wins += a
    b_wins += b
  end

  a_wins, b_wins
end

function part1(game)
  game = play_game(game, die_deterministic, 1000)
  minimum(game.scores) * game.die_rolls
end

function part2(game)
  maximum(play_game_quantum(game.positions[1], game.scores[1], game.positions[2], game.scores[2]))
end


game = load_game(ARGS[1])

println(game)
println("Part 1: ", part1(game))
println("Part 2: ", part2(game))
