defmodule State do
  @type t :: %__MODULE__{
          floors: list(any()),
          elevator_location: integer()
        }
  @type direction() :: :up | :down
  defstruct [:floors, :elevator_location]

  # apply the states
  @spec apply(t(), {direction(), list(any())}) :: t()
  def apply(state, action)

  def apply(state, {:up, items}) do
    current_elevator_location = state.elevator_location
    new_elevator_location = current_elevator_location + 1

    updated_floors =
      state.floors
      |> List.update_at(current_elevator_location, &Floor.remove(&1, items))
      |> List.update_at(new_elevator_location, &Floor.add(&1, items))

    %State{state | elevator_location: new_elevator_location, floors: updated_floors}
  end

  def apply(state, {:down, items}) do
    current_elevator_location = state.elevator_location
    new_elevator_location = current_elevator_location.elevator_location - 1

    updated_floors =
      state.floors
      |> List.update_at(current_elevator_location, &Floor.remove(&1, items))
      |> List.update_at(new_elevator_location, &Floor.add(&1, items))

    %State{state | elevator_location: new_elevator_location, floors: updated_floors}
  end

  def is_valid?(%State{floors: floors}) do
    Enum.all?(floors, &Floor.is_valid?/1)
  end

  # `next_states/1` returns, given a state, all its valid next states.
  def valid_next_states(%State{} = state) do
    state
    |> possible_actions()
    |> Enum.map(&State.apply(state, &1))
    |> Enum.filter(&State.is_valid?/1)
  end

  defp possible_actions(%State{} = state) do
    # grab the slots of the floor we're currently on.
    %Floor{slots: slots} = get_current_floor(state)

    # Grab all possible moveable items permutated with minimum length of one
    # and a maximum length of two.
    moveable_items = Permutate.list(slots)

    # Depending on the floor the elevator is currently on, we have limited
    # options to move either up, down or both. We describe possible actions
    # using tuples. eg:
    #
    # {:up, [:a, :b]} takes the items :a and :b up one floor, and;
    # {:down, [:z, :x]} will take the items :z and :x down one floor.
    case state.elevator_location do
      # when we're at the bottom floor - we can only move up.
      0 ->
        wrap_action(:up, moveable_items)

      3 ->
        # When we're at the top floor - we can only move down.
        wrap_action(:down, moveable_items)

      _ ->
        # every other floor since we can go up and down from here.
        wrap_action(:up, moveable_items) ++ wrap_action(:down, moveable_items)
    end
  end

  defp wrap_action(direction, moves),
    do:
      Enum.map(moves, fn moveables ->
        {direction, moveables}
      end)

  defp get_current_floor(%State{floors: floors, elevator_location: elevator_location} = state) do
    Enum.at(floors, elevator_location)
  end
end
