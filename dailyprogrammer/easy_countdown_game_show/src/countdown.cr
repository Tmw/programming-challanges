# solved Countdown challange
# https://www.reddit.com/r/dailyprogrammer/comments/6fe9cv/20170605_challenge_318_easy_countdown_game_show/
require "./countdown/*"

module Countdown
  class Solver

    property input : Array(Int32)
    property target : Int32

    def initialize(input)
      exit "Input should be formatted as: 1 2 3 4 5 6 123" unless input.size == 7
      @input = input.map { |p| p.to_i }
      @target = @input.pop
    end

    def start
      puts "Trying to find #{target} with input #{input}..."

      # calculate various permutations for inputs and operators
      operator_permutations = ["+", "-", "/", "*"].repeated_permutations(5)
      input_permutations = input.each_permutation

      # iterate over all permutations
      operator_permutations.each do |ops|
        input_permutations.rewind.each do |inp|
          # zip the two together to form a formula
          formula = inp.zip?(ops).reduce(Array(String|Int32).new) do |acc, item|
            acc << item[0]
            if operator = item[1]
              acc << operator
            end
            acc
          end

          # TODO: Probably refactor how the final answer is calculated
          rest = formula.clone
          while rest.size > 1
            rest = solve(rest)
          end

          answer = rest.first
          if answer == target
            puts formula.join("") + "=" + answer.to_s
            exit
          end

        end

      end

    end

    def solve(parts : Array(String | Int32)) : Array(String | Int32)
      left_hand  = parts.shift.to_i
      operator   = parts.shift
      right_hand = parts.shift.to_i

      answer = case operator
      when "+"
        left_hand + right_hand
      when "-"
        left_hand - right_hand
      when "/"
        left_hand / right_hand
      when "*"
        left_hand * right_hand
      end

      parts.unshift(answer.not_nil!)
    end

    private def exit(message)
      puts message
      exit
    end

  end
end

solver = Countdown::Solver.new ARGV
solver.start
