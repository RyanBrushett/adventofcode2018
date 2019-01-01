#!/usr/bin/env ruby

path = File.join(File.dirname(__FILE__), 'input')
input = File.open(path)
start = 0

input.each_line { |line| start += line.strip.to_i }
puts start

