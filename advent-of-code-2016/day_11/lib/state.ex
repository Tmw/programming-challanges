defmodule State do
  @type t :: %__MODULE__{
          floors: list(any()),
          elevator_location: integer(),
          hops: integer(),
          cost: integer(),
          hash: String.t()
        }
  @type direction() :: :up | :down
  defstruct [:floors, :elevator_location, hops: 0, cost: 999, hash: ""]

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

    new_state = %State{
      state
      | elevator_location: new_elevator_location,
        floors: updated_floors,
        hops: state.hops + 1
    }

    new_state
    |> Map.update(:cost, 0, fn _ -> State.cost(new_state) end)
    |> Map.update(:hash, "", fn _ -> State.hash(new_state) end)
  end

  def apply(state, {:down, items}) do
    current_elevator_location = state.elevator_location
    new_elevator_location = current_elevator_location - 1

    updated_floors =
      state.floors
      |> List.update_at(current_elevator_location, &Floor.remove(&1, items))
      |> List.update_at(new_elevator_location, &Floor.add(&1, items))

    new_state = %State{
      state
      | elevator_location: new_elevator_location,
        floors: updated_floors,
        hops: state.hops + 1
    }

    new_state
    |> Map.update(:cost, 0, fn _ -> State.cost(new_state) end)
    |> Map.update(:hash, "", fn _ -> State.hash(new_state) end)
  end

  def is_end?(%State{} = state) do
    # is elevator + all items on the fourth floor? 
    # a.k.a. All but top floor is empty?
    top_floor = length(state.floors) - 1
    semi_top_floor = top_floor - 1

    all_but_top_floor_empty? =
      state.floors
      |> Enum.slice(0..semi_top_floor)
      |> Enum.all?(&Floor.is_empty?/1)

    state.elevator_location == top_floor && all_but_top_floor_empty?
  end

  def is_valid?(%State{floors: floors}) do
    Enum.all?(floors, &Floor.is_valid?/1)
  end

  @spec hash(t()) :: String.t()
  @doc """
  `hash/1` returns a hash of the current state. The hash considers state like elevator position
  and the contents of the various floors. The end result is a SHA265 hash of these combined.
  """
  def hash(%State{} = state) do
    # hash function to make a relatively cheap hash of the passed in datastructure.
    floors =
      Enum.map(Enum.with_index(state.floors), fn {floor, floor_no} ->
        "F#{floor_no}:#{Hashable.hash(floor)}"
      end)

    Base.encode16(:crypto.hash(:sha256, "#{state.elevator_location}:#{floors}"))
  end

  # `next_states/1` returns, given a state, all its valid next states.
  def valid_next_states(%State{} = state) do
    state
    |> possible_actions()
    |> Enum.map(&State.apply(state, &1))
    |> Enum.filter(&State.is_valid?/1)
  end

  @spec goal_distance(t()) :: integer()
  def goal_distance(%State{} = state) do
    # How much would it cost to reach the end from here?
    # Naively.. 2 times floor number * non empty slots per floor?

    top_floor = length(state.floors) - 1

    state.floors
    |> Enum.with_index()
    |> Enum.reduce(0, fn {floor, floor_index}, acc ->
      floor_steps_from_top = top_floor - floor_index

      # max aantal items at the floor - 1 because minimum of one
      # is required to operate the elevator.
      items_at_floor = max(0, MapSet.size(floor.slots) - 1)

      # multiply by two because we're traveling the same distance
      # twice to bring the elevator back to the same floor for
      # subsequent items.
      new_res = floor_steps_from_top * 2 * items_at_floor

      new_res + acc
    end)
  end

  @spec start_distance(t()) :: integer()
  def start_distance(%State{hops: hops}), do: hops

  @spec cost(t()) :: integer()
  def cost(%State{} = state) do
    start_distance(state) + goal_distance(state)
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

  defp get_current_floor(%State{floors: floors, elevator_location: elevator_location}) do
    Enum.at(floors, elevator_location)
  end
end
