defmodule Microchip do
  defstruct [:identifier]
end

defimpl Validity, for: Microchip do
  @moduledoc """
  A microchip is fried when it is in the same room as a generator
  of another kind, unless its own type generator is present.
  """

  def valid?(%Microchip{identifier: identifier}, neighbours) do
    # if the chips is paired with its own generator, all is fine.
    contains_generator_of_own_kind =
      Enum.any?(neighbours, fn
        %Generator{identifier: neighbour_identifier} ->
          neighbour_identifier == identifier

        _ ->
          false
      end)

    # else; if the chip is in the same room with _only_ chips, we're ok
    exclusively_contains_chips =
      Enum.all?(neighbours, fn
        %Microchip{} -> true
        %Generator{} -> false
      end)

    cond do
      exclusively_contains_chips ->
        true

      contains_generator_of_own_kind ->
        true

      true ->
        # but if we're in the same room as a generator of another kind
        # and we do not have our own generator present - we break.
        false
    end
  end
end

defimpl Hashable, for: Microchip do
  def hash(%Microchip{identifier: identifier}) do
    "m-#{identifier}"
  end
end
