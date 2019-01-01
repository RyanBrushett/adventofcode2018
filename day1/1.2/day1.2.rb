#!/usr/bin/env ruby

path = File.join(File.dirname(__FILE__), 'input')
input = File.open(path)
values = {}
value = 0
values[value] = true

loop do
  input.rewind if input.eof?

  value += input.readline.strip.to_i

  if values[value]
    puts value
    break
  end

  values[value] = true
end

