# https://adventofcode.com/2021/day/16


function load_packets(filename)
  fp = open(filename)
  lines = readlines(fp)
  close(fp)

  lines
end

struct Literal
  version
  type_id
  value
end

struct Operator
  version
  type_id
  args
end

function decode_packet(value)
  result = ""

  for v in value
    result *= join(reverse(digits(parse(Int, v, base=16), base=2, pad=4)))
  end

  result
end

function b2i(value)
  parse(Int, value, base=2)
end

function chop_str(value, num_characters)
  value[num_characters + 1:length(value)]
end

TYPE_ID_LITERAL = 4
function consume_literal(version, type_id, packet)
  value = ""
  bits_consumed = 6

  while true
    is_last_chunk = packet[1] == '0'
    value *= packet[2:5]
    packet = chop_str(packet, 5)
    bits_consumed += 5

    if is_last_chunk
      break
    end
  end

  Literal(version, TYPE_ID_LITERAL, b2i(value)), packet, bits_consumed
end

function consume_operator(version, type_id, packet)
  length_type_id = packet[1]
  bits_consumed = 7
  args = []

  if length_type_id == '0'
    total_length = b2i(packet[2:16])
    bits_consumed += 15
    packet = chop_str(packet, 16)

    arg_bits_consumed = 0

    while arg_bits_consumed < total_length
      res, packet, consumed = consume_packet(packet)
      push!(args, res)
      arg_bits_consumed += consumed
    end

    bits_consumed += arg_bits_consumed

  elseif length_type_id == '1'
    num_sub_packets = b2i(packet[2:12])
    bits_consumed += 11
    packet = chop_str(packet, 12)

    for i in 1:num_sub_packets
      res, packet, consumed = consume_packet(packet)
      push!(args, res)
      bits_consumed += consumed
    end

  end

  Operator(version, type_id, args), packet, bits_consumed
end

function consume_packet(packet)
  version = b2i(packet[1:3])
  type_id = b2i(packet[4:6])
  packet = chop_str(packet, 6)

  if type_id == TYPE_ID_LITERAL
    return consume_literal(version, type_id, packet)
  else
    return consume_operator(version, type_id, packet)
  end
end

function sum_versions(expression::Literal)
  expression.version
end

function sum_versions(expression::Operator)
  expression.version + sum(map(arg -> sum_versions(arg), expression.args))
end

function evaluate(expression::Literal)
  expression.value
end

function evaluate(expression::Operator)
  args = map(arg -> evaluate(arg), expression.args)

  if expression.type_id == 0
    return sum(args)
  elseif expression.type_id == 1
    return reduce(*, args)
  elseif expression.type_id == 2
    return minimum(args)
  elseif expression.type_id == 3
    return maximum(args)
  elseif expression.type_id == 5
    return Int(args[1] > args[2])
  elseif expression.type_id == 6
    return Int(args[1] < args[2])
  elseif expression.type_id == 7
    return Int(args[1] == args[2])
  end
end


packets = load_packets(ARGS[1])

for packet in packets
  println(packet)
  result = consume_packet(decode_packet(packet))
  # println(result)
  println("Version sum: ", sum_versions(result[1]))
  println("Result: ", evaluate(result[1]))
end
