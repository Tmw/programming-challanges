defmodule Generator do
  defstruct [:identifier]
end

defimpl Validity, for: Generator do
  def valid?(%Generator{identifier: _identifier} = _self, _neighbours) do
    # we are a generator - we don't care.
    true
  end
end

defimpl Hashable, for: Generator do
  def hash(%Generator{identifier: identifier}) do
    "g-#{identifier}"
  end
end
