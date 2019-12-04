defmodule Renderer do
  def render(%State{floors: floors} = state) do
    # take the floors from the passed in state. Flip the order so Floor 1
    # will be at the bottom and assign indexes, starting at one.
    floors =
      floors
      |> Enum.reverse()
      |> Enum.with_index()

    # for each floor, print the slots. If the current floor is equal to the
    # floor that the elevator is currently on, print a capital E. Else
    # print a white space.
    #
    num_floors = length(floors)

    for {floor, i} <- floors do
      current_floor = num_floors - i - 1
      IO.binwrite(:stdio, "F#{current_floor} ")

      if state.elevator_location == current_floor do
        IO.binwrite(:stdio, " E ")
      else
        IO.binwrite(:stdio, "   ")
      end

      render_floor(floor)
      IO.binwrite(:stdio, "\n")
    end

    cost = State.cost(state)

    IO.binwrite(:stdio, "cost for node: #{cost}")
  end

  defp render_floor(%Floor{slots: slots}) do
    for slot <- slots, do: render_slot_contents(slot)
  end

  defp render_slot_contents(%Generator{identifier: id}),
    do: render_slot_contents(id, :generator)

  defp render_slot_contents(%Microchip{identifier: id}),
    do: render_slot_contents(id, :microchip)

  defp render_slot_contents(identifier, type) when is_atom(identifier) do
    id =
      identifier
      |> Atom.to_string()
      |> String.upcase()
      |> String.slice(0..2)

    type =
      type
      |> Atom.to_string()
      |> String.upcase()
      |> String.at(0)

    IO.binwrite(:stdio, " #{id}-#{type} ")
  end
end
