# https://adventofcode.com/2021/day/4


mutable struct Cell
  value
  seen
end

Cell(value) = Cell(value, false)

function load_game(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  draws = map(x -> parse(Int32, x), split(lines[1], ','))

  boards = Matrix[]
  i = 3
  while i < length(lines)
    board = Matrix{Cell}(undef, 5, 5)
    for row in 1:5
      col = 1
      for value in map(x -> parse(Int32, x), split(strip(lines[i + (row - 1)])))
        board[row, col] = Cell(value)
        col += 1
      end
    end
    push!(boards, board)

    i += 6
  end

  [boards, draws]
end

function mark_cell(board, draw)
  for row in eachrow(board)
    for cell in row
      if cell.value == draw
        cell.seen = true
      end
    end
  end
end

function board_wins(board)
  for row in eachrow(board)
    if all(map(x -> x.seen, row))
      return true
    end
  end

  for col in eachcol(board)
    if all(map(x -> x.seen, col))
      return true
    end
  end
  
  false
end

function tally_unmarked(board)
  tally = 0

  for row in eachrow(board)
    tally += reduce(+, map(x -> x.seen ? 0 : x.value, row))
  end

  tally
end

function play_to_win(boards, draws)
  for draw in draws
    for b in 1:length(boards)
      mark_cell(boards[b], draw)

      if board_wins(boards[b])
        score = tally_unmarked(boards[b]) * draw
        return [b, score]
      end
    end
  end
end

function play_to_lose(boards, draw)
  winners = Int32[]

  for draw in draws
    for b in 1:length(boards)
      if findfirst(x -> x == b, winners) == nothing
        mark_cell(boards[b], draw)

        if board_wins(boards[b])
          push!(winners, b)

          if length(boards) == length(winners)
            score = tally_unmarked(boards[b]) * draw
            return [b, score]
          end
        end
      end
    end
  end
end


boards, draws = load_game(ARGS[1])
winner, score = play_to_win(boards, draws)
println("First Winner: ", winner)
println("First Winner Score: ", score)
winner, score = play_to_lose(boards, draws)
println("Last Winner: ", winner)
println("Last Winner Score: ", score)
