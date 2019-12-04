defmodule Day11 do
  @moduledoc """
  - [x] The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
  - [x] The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
  - [x] The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
  - [x] The fourth floor contains nothing relevant.
  """
  def solve do
    state = get_initial_state()
    search([state])
  end

  def search([]), do: :end_not_found

  def search([first | rest] = _open, visited \\ MapSet.new()) do
    first_hash = State.hash(first)

    cond do
      State.is_end?(first) ->
        {:found, first}

      MapSet.member?(visited, first_hash) ->
        # already seen this node; skipping.
        search(rest, visited)

      true ->
        # Keep looking..
        # Pick the nodes that are _connected_ to this one and filter out the ones we already visited.
        next_hops =
          State.valid_next_states(first)
          |> Enum.reject(&MapSet.member?(visited, State.hash(&1)))

        # put the current node in the _visited_ list
        visited = MapSet.put(visited, first_hash)

        # new_open = Enum.sort_by(next_hops ++ rest, &Map.get(&1, :cost))
        # Enum.take(new_open, 5)
        # |> Enum.map(&Map.get(&1, :cost))
        # |> IO.inspect(label: "first 5 costs")

        new_open = next_hops ++ rest
        # IO.puts("Keep looking.. #{length(new_open)} options in open list..")

        search(new_open, visited)
    end
  end

  # TODO: Make sure the `open` list will be ordered by `cost` before recursing.

  # TODO: Have a concept of a trail so we can track which nodes it came from so we can
  # succesfully work our way back and figure out the shortest route.

  def get_initial_state do
    # hardcoded the initial state since parsing the input is not the challanging
    # part of this puzzle..
    %State{
      elevator_location: 0,
      floors: [
        Floor.new([
          %Generator{identifier: :thulium},
          %Microchip{identifier: :thulium},
          %Generator{identifier: :plutonium},
          %Generator{identifier: :strontium}
        ]),
        Floor.new([
          %Microchip{identifier: :plutonium},
          %Microchip{identifier: :strontium}
        ]),
        Floor.new([
          %Generator{identifier: :promethium},
          %Microchip{identifier: :promethium},
          %Generator{identifier: :ruthenium},
          %Microchip{identifier: :ruthenium}
        ]),
        Floor.new([])
      ]
    }
  end
end
