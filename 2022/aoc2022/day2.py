# https://adventofcode.com/2022/day/2

from enum import IntEnum

from .util import get_lines


class Weapon(IntEnum):
  ROCK = 1
  PAPER = 2
  SCISSORS = 3

class Result(IntEnum):
  WIN = 6
  DRAW = 3
  LOSS = 0

InputMapping = dict[str, Weapon]
StrategyMapping = dict[str, Weapon | Result]
Round = tuple[Weapon, Weapon]


def determine_play(opponent: Weapon, result: Result) -> Weapon:
  if result == Result.DRAW:
    return opponent
  
  if result == Result.WIN:
    return {
      Weapon.ROCK: Weapon.PAPER,
      Weapon.PAPER: Weapon.SCISSORS,
      Weapon.SCISSORS: Weapon.ROCK
    }[opponent]

  return {
    Weapon.ROCK: Weapon.SCISSORS,
    Weapon.PAPER: Weapon.ROCK,
    Weapon.SCISSORS: Weapon.PAPER
  }[opponent]

def get_rounds(data: list[str], input_map: InputMapping, strategy_map: StrategyMapping) -> list[Round]:
  rounds: list[Round] = []

  for line in data:
    plays = line.split(' ', 2)

    opponent = input_map[plays[0]]
    strat = strategy_map[plays[1]]

    if isinstance(strat, Result):
      you = determine_play(opponent, strat)
    else:
      you = strat

    rounds.append((opponent, you))

  return rounds

def get_result(round: Round) -> Result:
  if round[0] == round[1]:
    return Result.DRAW

  if (round[1] == Weapon.ROCK and round[0] == Weapon.SCISSORS) or \
      (round[1] == Weapon.PAPER and round[0] == Weapon.ROCK) or \
      (round[1] == Weapon.SCISSORS and round[0] == Weapon.PAPER):
    return Result.WIN

  return Result.LOSS
  
def score_round(round: Round) -> int:
  result = get_result(round)
  return result.value + round[1].value

def score_game(rounds: list[Round]) -> int:
  return sum([
    score_round(round)
    for round in rounds
  ])


INPUT_MAP: InputMapping = {
  'A': Weapon.ROCK,
  'B': Weapon.PAPER,
  'C': Weapon.SCISSORS
}

STRATEGY_MAP_PART_1: StrategyMapping = {
  'X': Weapon.ROCK,
  'Y': Weapon.PAPER,
  'Z': Weapon.SCISSORS
}

STRATEGY_MAP_PART_2: StrategyMapping = {
  'X': Result.LOSS,
  'Y': Result.DRAW,
  'Z': Result.WIN
}

data = get_lines('2/input.txt')

rounds = get_rounds(data, INPUT_MAP, STRATEGY_MAP_PART_1)
print(f"Part 1 Total Score: {score_game(rounds)}")

rounds = get_rounds(data, INPUT_MAP, STRATEGY_MAP_PART_2)
print(f"Part 2 Total Score: {score_game(rounds)}")
