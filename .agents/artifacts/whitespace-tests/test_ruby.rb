#!/usr/bin/env ruby

# Test Ruby heredoc whitespace stripping

puts "=== Ruby Squiggly Heredoc (<<~) ==="
text = <<~TEXT
  Line 1 with 2 spaces
  Line 2 with 2 spaces
TEXT

puts "Length: #{text.length}"
puts "Content: #{text.inspect}"
puts "Bytes: #{text.bytes.inspect}"
puts

puts "=== Ruby Plain Heredoc (<<) ==="
text2 = <<TEXT
  Line 1 with 2 spaces
  Line 2 with 2 spaces
TEXT

puts "Length: #{text2.length}"
puts "Content: #{text2.inspect}"
puts "Bytes: #{text2.bytes.inspect}"
puts

puts "=== Ruby Squiggly with 4 spaces ==="
text3 = <<~TEXT
    Line 1 with 4 spaces
    Line 2 with 4 spaces
TEXT

puts "Length: #{text3.length}"
puts "Content: #{text3.inspect}"
puts

puts "=== Ruby Squiggly with mixed indentation ==="
text4 = <<~TEXT
  Base indentation (2 spaces)
    Extra indented (4 spaces)
  Back to base
TEXT

puts "Content: #{text4.inspect}"
