#!/usr/bin/env ruby

input = File.open('input')
start = 0

input.each_line { |line| start += line.strip.to_i }
puts start

