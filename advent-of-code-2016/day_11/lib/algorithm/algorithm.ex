defmodule Day11.Algorithm do
  @type optional_visited_mapset() :: list(any()) | nil
  @type root :: list(State.t())

  @callback search(root(), optional_visited_mapset()) ::
              {:found, State.t()} | :end_not_found
end
