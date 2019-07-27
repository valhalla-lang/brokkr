require 'yaml'

def wrap(arr)
  '| ' + (arr.join ' | ') + ' |'
end

LIST = YAML.load File.read ARGV[0]
HEAD = wrap %w(Byte Name Operands Description)

rows = Array.new

LIST.each do |row|
  fields = Array.new
  fields << "`#{row[:byte].to_s(2).rjust(8, '0')}`"
  fields << "`#{row[:name].upcase}`"

  operand_desc = Array.new
  row[:operands].times do |i|
    operand_desc << "#{i + 1} — #{row[:operand_desc][i]}"
  end

  fields << operand_desc.join('<br />')
  fields << row[:desc]
  
  rows << (wrap fields)
end

LINE = "\n|:---:|:---:|---|---|\n"
TABLE = HEAD + LINE + rows.join("\n")

puts "# Brokkr Bytecode Specification"
puts TABLE
