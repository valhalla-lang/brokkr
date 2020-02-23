#!/usr/bin/env ruby

require 'yaml'

def wrap(arr)
  '| ' + (arr.join ' | ') + ' |'
end

LIST = YAML.load File.read ARGV[0]
HEAD = wrap %w(Byte Name Operands Description)

rows = Array.new

LIST.each do |row|
  fields = Array.new
  fields << "`0x#{row[:byte].to_s(16).rjust(2, '0')}`<br />(#{row[:byte]})"
  fields << "`#{row[:name].upcase}`"

  operand_desc = Array.new
  row[:operands].times do |i|
    operand_desc << "#{i + 1} — #{row[:operand_desc][i]}"
  end

  operand_desc = ['<center>N/A</center>'] if operand_desc.empty?

  fields << operand_desc.join('<br />')
  fields << row[:desc]

  rows << (wrap fields)
end

LINE = "\n|:---:|:---:|---|---|\n"
TABLE = HEAD + LINE + rows.join("\n")

KEY = <<EOF

Sizes are as follows:

| Type     | Size                     |
|----------|--------------------------|
| Operator | 1 Byte  (8  Bits,  `u8`) |
| Operand  | 2 Bytes (16 Bits, `u16`) |

EOF

puts "# Brokkr Bytecode Specification"
puts KEY
puts "## Bytecodes\n"
puts TABLE
