defmodule Day11.Algorithm.AStar do
  @behaviour Day11.Algorithm

  @type state_hash() :: String.t()
  @type visited_map() :: %{
          state_hash() => integer()
        }

  @impl true
  def search(open, discovered \\ Map.new())
  def search([], _visited), do: :end_not_found

  def search([current | rest] = _open, discovered) do
    with {:progress, current} <- check_progress(current),
         successors <- get_successors(discovered, current),
         {:not_found, successors} <- successors_contain_goal(successors) do
      # mark successors found as discovered or update their cost value if 
      # they were already present.
      discovered = update_discovered_map_with_successors(discovered, successors)

      # sort the open queue by cost and recurse
      open = Enum.sort_by(successors ++ rest, &Map.get(&1, :cost))

      search(open, discovered)
    else
      {:goal, current} -> {:found, current}
    end
  end

  # given the discovered map and the found successors that are known to be either
  # unique (not previously discovered), *or* cheaper (lower cost value) than
  # the node we previously found, it will return an updated discovered_map with
  # these updates reflected
  defp update_discovered_map_with_successors(discovered_map, successors) do
    Enum.reduce(successors, discovered_map, fn successor, discovered ->
      Map.update(discovered, successor.hash, successor.cost, fn _oldcost ->
        successor.cost
      end)
    end)
  end

  # given a node, it'll return its neighbouring nodes filtering out
  # the ones that are already discovered, unless the newly discovered one
  # has a lower cost value than the one we already explored
  defp get_successors(discovered_map, current) do
    # explore neighbouring nodes. and filter potential _duplicate_ nodes
    # by their cost compared to their previous discovered cost
    current
    |> State.valid_next_states()
    |> Enum.reject(already_discovered_with_lower_cost(discovered_map))
  end

  # returns either `{:not_found, list()}` where list is the original passed
  # in list, or {:goal, State.t()} when one of the successors is the goal
  defp successors_contain_goal(successors) do
    case Enum.find(successors, &State.is_end?/1) do
      nil -> {:not_found, successors}
      goal -> {:goal, goal}
    end
  end

  # returns a filter function that will return a boolean if the given node
  # was already present in the discovered_map *and* has a lower cost value
  # in said discovered map
  defp already_discovered_with_lower_cost(discovered_map) do
    fn node ->
      Map.has_key?(discovered_map, node.hash) &&
        node.cost >= Map.get(discovered_map, node.hash)
    end
  end

  # is the passed in node considered the goal or is it considered progress
  defp check_progress(state) do
    case State.is_end?(state) do
      true -> {:goal, state}
      false -> {:progress, state}
    end
  end
end
