defmodule Day11.Algorithm.BreadthFirstSearch do
  def search(current, visited \\ MapSet.new())
  def search([], _visited), do: :end_not_found

  def search([current | rest] = _open, discovered) do
    cond do
      State.is_end?(current) ->
        # NOTE: For part A it should find it 31 hops in.
        {:found, current}

      MapSet.member?(discovered, current.hash) ->
        # seen this node already in a previous pass
        # ignore and continue with the tail of the open nodes.
        search(rest, discovered)

      true ->
        # new node; grab all its successors (connected nodes) and
        # drop the ones we for sure already saw.
        new_successors =
          current
          |> State.valid_next_states()
          |> Enum.reject(&MapSet.member?(discovered, &1.hash))

        # then add the current not to the list of discovered nodes
        # and recurse with the new list comprised of rest + successors
        discovered = MapSet.put(discovered, current.hash)
        search(rest ++ new_successors, discovered)
    end
  end
end
