defmodule Day11 do
  @moduledoc """
  - [x] The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
  - [x] The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
  - [x] The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
  - [x] The fourth floor contains nothing relevant.
  """
  def solve do
    solve(:part_a)
    solve(:part_b)
  end

  def solve(part) do
    state = [PuzzleInput.for(part)]

    ts_begin = NaiveDateTime.utc_now()
    {:found, goal} = search(state, Day11.Algorithm.AStar)
    ts_end = NaiveDateTime.utc_now()
    duration = NaiveDateTime.diff(ts_end, ts_begin, :millisecond)

    IO.puts("Found answer to #{part} in #{duration} ms. Answer is = #{goal.hops} steps")
  end

  defp search(root, algorithm) do
    algorithm.search(root)
  end
end
