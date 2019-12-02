defmodule Day10.Node.Configuration do
  @moduledoc """
  %Configuration{} struct describes where a node should send its
  high and its low value(s) to. Destination is described as a String
  that consists of the node type (bot or output) and its unique identifier.
  """
  @type destination() :: String.t()
  @type t() :: %__MODULE__{
          low_destination: destination(),
          high_destination: destination()
        }
  @enforce_keys [:low_destination, :high_destination]
  defstruct([:low_destination, :high_destination])

  def new(low_destination, high_destination) do
    %__MODULE__{
      low_destination: low_destination,
      high_destination: high_destination
    }
  end
end

defmodule Day10.Node.State do
  @moduledoc """
  `%State{}` describes the current state of the node. In practice that means
  the chips is collected and optionally a `%Configuration{}`.
  """
  alias Day10.Node.Configuration

  @type chips() :: list(integer())
  @type t() :: %__MODULE__{
          identifier: String.t(),
          chips: chips(),
          configuration: Configuration.t() | nil
        }
  defstruct identifier: nil, chips: [], configuration: nil

  @spec new(String.t(), Configuration.t() | nil) :: t()
  def new(identifier, configuration \\ nil) do
    %__MODULE__{
      identifier: identifier,
      chips: [],
      configuration: configuration
    }
  end
end

defmodule Day10.Node do
  @min_chip_count 2

  # which chips should trigger the probe
  @probe [17, 61]
  use GenServer

  alias Day10.Node.{Configuration, State}

  def child_spec(opts) do
    %{
      id: Keyword.get(opts, :identifier),
      start: {__MODULE__, :start_link, [opts]}
    }
  end

  def start_link(_args, opts) do
    identifier = Keyword.get(opts, :identifier)
    type = Keyword.get(opts, :type)
    configuration = Keyword.get(opts, :configuration)

    state = State.new(identifier, configuration)
    opts = [name: node_name(type, identifier)]
    GenServer.start_link(__MODULE__, state, opts)
  end

  @impl true
  def init(args) do
    {:ok, args}
  end

  @impl true
  def handle_call({:receive, chipnumber}, _from, state) do
    # store the received chip.
    state = Map.put(state, :chips, [chipnumber | state.chips])

    # run the chips distribution step if the node is legible to do so.
    if should_distribute_chips(state) do
      distribute_chips(state)
    end

    {:reply, :ok, state}
  end

  @impl true
  def handle_call(:list, _from, state) do
    {:reply, state.chips, state}
  end

  # only distribute the chips if the node is configured to do so and we
  # have collected up-to n chips.
  defp should_distribute_chips(state) do
    state.configuration != nil && length(state.chips) >= @min_chip_count
  end

  defp distribute_chips(state) do
    [lowest, highest] = Enum.sort(state.chips)

    %Configuration{
      low_destination: low_destination,
      high_destination: high_destination
    } = state.configuration

    if [lowest, highest] == @probe do
      IO.puts("bot #{state.identifier} is responsible for comparing #{lowest} and #{highest}")
    end

    # redistribute the received chips accordingly
    receive(low_destination, lowest)
    receive(high_destination, highest)
  end

  # public API
  def receive(bot, chipnumber) do
    GenServer.call(bot, {:receive, chipnumber})
  end

  def list(bot) do
    GenServer.call(bot, :list)
  end

  def node_name(type, identifier) do
    Module.concat(__MODULE__, "#{type}-#{identifier}")
  end
end
