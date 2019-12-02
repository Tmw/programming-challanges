defmodule Day10 do
  @challange_input "lib/input.txt"

  alias Day10.{
    Parser,
    ParseResult,
    Node,
    Node.Configuration,
    Node.Supervisor
  }

  def solve do
    parsed = Parser.parse(read_input())

    # extract & spawn the output nodes
    outputs = extract_outputs(parsed)
    Supervisor.spawn_nodes(:output, outputs)

    # extract & spawn the bot nodes
    bots = extract_bots(parsed)
    Supervisor.spawn_nodes(:bot, bots)

    # once we've spawned the bots and outputs, assign the chips to the bots.
    process_assignments(parsed)

    # get answer of part B
    :output
    |> collect_chips_from_nodes([0, 1, 2])
    |> multiply()
    |> IO.inspect(label: "Answer of part B")
  end

  defp collect_chips_from_nodes(type, nodes) do
    nodes
    |> Enum.map(&Node.node_name(type, &1))
    |> Enum.flat_map(&Node.list/1)
  end

  defp multiply(list_of_integers) do
    Enum.reduce(list_of_integers, 1, fn integer, total -> integer * total end)
  end

  # process_assignments/1 takes the %ParseResult's assignments and processes
  # them by sending the correct values to the bots.
  defp process_assignments(%ParseResult{chip_assignments: assignments}) do
    for assignment <- assignments do
      node_name =
        Node.node_name(
          assignment.target_address.type,
          assignment.target_address.identifier
        )

      Node.receive(node_name, assignment.value)
    end
  end

  defp extract_bots(%ParseResult{nodes: bots}) do
    bots
    |> uniqify_by(:identifier)
    |> assign_bot_configuration()
  end

  # iterate over the bots and parse their `%Configuration{}`. Convert into
  # a list with two element tuples so the bots can be spawned with their
  # associated configurations.
  defp assign_bot_configuration(bots) do
    Enum.map(bots, fn bot ->
      high_destination =
        Node.node_name(
          bot.high_destination.type,
          bot.high_destination.identifier
        )

      low_destination =
        Node.node_name(
          bot.low_destination.type,
          bot.low_destination.identifier
        )

      configuration =
        Configuration.new(
          low_destination,
          high_destination
        )

      {bot, configuration}
    end)
  end

  # From the ParseResult get the nodes and return a list of nodes where type
  # equals `:output`. The list is made unique by looking at the identifier.
  defp extract_outputs(%ParseResult{nodes: nodes}) do
    nodes
    |> get_destination_addresses()
    |> filter_node_type(:output)
    |> uniqify_by(:identifier)
  end

  defp get_destination_addresses(nodes) do
    Enum.flat_map(nodes, fn node ->
      [node.low_destination, node.high_destination]
    end)
  end

  defp filter_node_type(nodes, type) do
    Enum.filter(nodes, fn node ->
      node.type == type
    end)
  end

  defp uniqify_by(nodes, field) do
    Enum.uniq_by(nodes, &Map.get(&1, field))
  end

  defp read_input(path \\ @challange_input) do
    File.read!(path)
  end
end
