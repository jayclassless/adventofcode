# https://adventofcode.com/2020/day/4

require 'set'


input_data = File.new(ARGV[0]).read

passports = input_data
  .split("\n\n")
  .map do |data|
    data
      .split
      .map { |d| d.split(":") }
      .to_h
  end


REQUIRED_FIELDS = [
  "byr",
  "iyr",
  "eyr",
  "hgt",
  "hcl",
  "ecl",
  "pid",
  "cid",
].to_set


valid_passports = passports
  .filter do |passport|
    fields = passport.keys.to_set

    if REQUIRED_FIELDS.subset?(fields)
      true
    else
      missing = REQUIRED_FIELDS - fields
      missing.to_a == ["cid"]
    end
  end

puts "Part 1: #{valid_passports.count}"

def valid_height(value)
  if value.match?(/^\d+(cm|in)$/)
    val = Integer(value[0...(value.length - 2)])
    unit = value[(value.length - 2)...]

    if unit == "cm" && val >= 150 && val <= 193
      return true
    elsif unit == "in" && val >= 59 && val <= 76
      return true
    end
  end

  false
end

really_valid_passports = valid_passports
  .filter do |passport|
    byr = Integer(passport["byr"])
    iyr = Integer(passport["iyr"])
    eyr = Integer(passport["eyr"])

    [
      byr >= 1920 && byr <= 2002,
      iyr >= 2010 && iyr <= 2020,
      eyr >= 2020 && eyr <= 2030,
      valid_height(passport["hgt"]),
      passport["hcl"].match?(/^#[a-f0-9]{6}$/),
      ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].include?(passport["ecl"]),
      passport["pid"].match?(/^[0-9]{9}$/),
    ].all?
  end

puts "Part 2: #{really_valid_passports.count}"
