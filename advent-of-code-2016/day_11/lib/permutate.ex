defmodule Permutate do
  def list(list) do
    list =
      for item_a <- list,
          item_b <- list,
          do: {item_a, item_b}

    list =
      Enum.map(list, fn permutation ->
        case permutation do
          {a, a} -> [a]
          {a, b} -> Enum.sort([a, b])
        end
      end)

    Enum.uniq(list)
  end
end
