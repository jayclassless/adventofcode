# https://adventofcode.com/2020/day/8

Instruction = Struct.new(
  :op,
  :arg
)

instructions = File
  .new(ARGV[0])
  .readlines
  .map do |line|
    op, arg = line.chomp.split(' ')
    Instruction.new(op, Integer(arg))
  end

def execute(instructions)
  acc = 0
  executed = []

  line = 0
  while line < instructions.length
    if executed.include?(line)
      return [acc, true]
    end
    executed << line

    if instructions[line].op == 'acc'
      acc += instructions[line].arg
      line += 1
    elsif instructions[line].op == 'jmp'
      line += instructions[line].arg
    elsif instructions[line].op == 'nop'
      line += 1
    else
      raise 'wtf'
    end
  end

  [acc, false]
end

def part1(instructions)
  execute(instructions)[0]
end

def part2(instructions)
  instructions.each_index do |i|
    if instructions[i].op == 'nop'
      to_test = Array.new(instructions)
      to_test[i] = Instruction.new('jmp', to_test[i].arg)
    elsif instructions[i].op == 'jmp'
      to_test = Array.new(instructions)
      to_test[i] = Instruction.new('nop', to_test[i].arg)
    else
      to_test = instructions
    end

    acc, inf_loop = execute(to_test)

    return acc unless inf_loop
  end
end


puts "Part 1: #{part1(instructions)}"
puts "Part 2: #{part2(instructions)}"
