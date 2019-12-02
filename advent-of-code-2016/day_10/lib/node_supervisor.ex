defmodule Day10.Node.Supervisor do
  use DynamicSupervisor

  alias Day10.Node.Configuration

  @impl true
  def init(args) do
    DynamicSupervisor.init(
      strategy: :one_for_one,
      extra_arguments: [args]
    )
  end

  def start_link(init_arg) do
    DynamicSupervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @type node_type() :: :bot | :output
  @type node_identifier() :: String.t()
  @spec start_node(node_type(), node_identifier()) :: {:ok, pid()}
  def start_node(type, identifier, configuration \\ nil) do
    spec =
      {Day10.Node,
       [
         identifier: identifier,
         type: type,
         configuration: configuration
       ]}

    DynamicSupervisor.start_child(__MODULE__, spec)
  end

  @spec spawn_nodes(node_type(), list(node_identifier() | {node_identifier(), Configuration.t()})) ::
          :ok
  def spawn_nodes(type, items) do
    Enum.each(items, fn
      {bot, configuration} -> start_node(type, bot.identifier, configuration)
      node -> start_node(type, node.identifier)
    end)
  end
end
