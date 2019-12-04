defmodule Floor do
  defstruct slots: MapSet.new()

  def new do
    Floor.new([])
  end

  def new(items) do
    %Floor{slots: MapSet.new(items)}
  end

  def add(%Floor{} = floor, items) when is_list(items) do
    slots = MapSet.union(floor.slots, MapSet.new(items))

    %Floor{floor | slots: slots}
  end

  def add(%Floor{} = floor, item) do
    add(floor, [item])
  end

  def remove(%Floor{} = floor, items) when is_list(items) do
    slots = MapSet.difference(floor.slots, MapSet.new(items))
    %Floor{floor | slots: slots}
  end

  def remove(%Floor{} = floor, item) do
    remove(floor, [item])
  end

  def is_valid?(%Floor{slots: slots}) do
    Enum.all?(slots, &Validity.valid?(&1, slots))
  end

  def is_empty?(%Floor{slots: slots}) do
    MapSet.size(slots) == 0
  end
end

defimpl Hashable, for: Floor do
  def hash(%Floor{slots: slots}) do
    slots
    |> Enum.map(&Hashable.hash/1)
    |> Enum.join()
  end
end
