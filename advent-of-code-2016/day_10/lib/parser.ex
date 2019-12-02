defmodule Day10.ParseResult.TargetAddress do
  @moduledoc """
  `%TargetAddress{}` describes the type and identifier of a target node
  in the system.
  """
  alias __MODULE__
  @type target_identifier() :: String.t()
  @type target_type() :: :bot | :output

  @type t :: %__MODULE__{identifier: target_identifier(), type: target_type()}
  @enforce_keys [:identifier, :type]
  defstruct [:identifier, :type]

  @spec new(target_type(), target_identifier()) :: TargetAddress.t()
  def new(type, identifier) when is_atom(type) do
    %__MODULE__{
      type: type,
      identifier: identifier
    }
  end

  @spec new(String.t(), target_identifier()) :: TargetAddress.t()
  def new("bot", identifier), do: new(:bot, identifier)
  def new("output", identifier), do: new(:output, identifier)

  @spec for_bot(target_identifier()) :: TargetAddress.t()
  def for_bot(identifier), do: new(:bot, identifier)

  @spec for_output(target_identifier()) :: TargetAddress.t()
  def for_output(identifier), do: new(:output, identifier)
end

defmodule Day10.ParseResult.Node do
  @moduledoc """
  `%Node{}` describes a instruction to configure a node in the system.
  """
  alias Day10.ParseResult.{Node, TargetAddress}

  @type node_identifier() :: String.t()
  @type t :: %__MODULE__{
          identifier: node_identifier(),
          low_destination: TargetAddress.t(),
          high_destination: TargetAddress.t()
        }
  @enforce_keys [:identifier, :low_destination, :high_destination]
  defstruct [:identifier, :low_destination, :high_destination]

  @spec new(node_identifier(), TargetAddress.t(), TargetAddress.t()) :: Node.t()
  def new(identifier, low_destination, high_destination) do
    %__MODULE__{
      identifier: identifier,
      low_destination: low_destination,
      high_destination: high_destination
    }
  end
end

defmodule Day10.ParseResult.ChipAssignment do
  @moduledoc """
  `%ChipAssignment{} describes a initial chip assignment instruction.
  """
  alias Day10.ParseResult.{ChipAssignment, TargetAddress}

  @type chip_value() :: integer()
  @type t :: %__MODULE__{
          value: chip_value(),
          target_address: TargetAddress.t()
        }

  @enforce_keys [:value, :target_address]
  defstruct [:value, :target_address]

  @spec new(chip_value(), TargetAddress.t()) :: ChipAssignment.t()
  def new(value, target_address) do
    %__MODULE__{
      value: value,
      target_address: target_address
    }
  end
end

defmodule Day10.ParseResult do
  @moduledoc """
  `%Parseresult{}` is a tree representation of the parsed instruction set. It 
  separates the items it found, which are either of type `%Node{}` or of type
  `%ChipAssignment{}`.
  """
  alias __MODULE__
  alias __MODULE__.{ChipAssignment, Node}

  @type chip_assignments() :: list(ChipAssignment.t())
  @type nodes() :: list(Node.t())
  @type t :: %__MODULE__{
          chip_assignments: chip_assignments(),
          nodes: nodes()
        }
  @enforce_keys [:chip_assignments, :nodes]
  defstruct [:chip_assignments, :nodes]

  @doc """
  `new/0` initializes a new _empty_ `%ParseResult{}`.
  """

  @spec new() :: __MODULE__.t()
  def new do
    %__MODULE__{
      nodes: [],
      chip_assignments: []
    }
  end

  @doc """
  `add_node/2` takes a `%ParseResult{}` and a `%Node{}` and returns the updated
  `%ParseResult{}` with the passed in `%Node{}` appended to the list of nodes.
  """

  @spec add_node(ParseResult.t(), Node.t()) :: ParseResult.t()
  def add_node(%ParseResult{nodes: nodes} = result, %Node{} = node) do
    nodes = Enum.reverse([node | nodes])
    %ParseResult{result | nodes: nodes}
  end

  @doc """
  `add_assignment/2` takes a `%ParseResult{}` and a `%ChipAssignment{}` and 
  returns the updated `%ParseResult{}` with the passed in `%ChipAssignment{}` 
  appended to the list of chip_assignments.
  """
  @spec add_assignment(ParseResult.t(), ChipAssignment.t()) :: ParseResult.t()
  def add_assignment(
        %ParseResult{chip_assignments: assignments} = result,
        %ChipAssignment{} = assignment
      ) do
    assignments = Enum.reverse([assignment | assignments])
    %ParseResult{result | chip_assignments: assignments}
  end
end

defmodule Day10.Parser do
  @moduledoc """
  Day10.Parser takes the challange input and parses it into a `%ParseResult{}`
  struct which separas the found nodes (either bots or outputs) and its chip
  assignments, ready to be processed later on.
  """
  @assignment_regex ~r/value (\d+) goes to bot (\d+)/
  @bot_regex ~r/bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)/

  alias Day10.ParseResult
  alias ParseResult.{Node, ChipAssignment, TargetAddress}

  @doc """
  `parse/1` takes the input as a string and parses it as a `%ParseResult{}`
  struct if all went well. It raises an exception when it encounters an invalid
  instruction.
  """
  @spec parse(String.t()) :: ParseResult.t()
  def parse(contents) do
    contents
    |> String.split("\n", trim: true)
    |> to_parse_result()
  end

  @spec to_parse_result(list(String.t())) :: ParseResult.t()
  defp to_parse_result(lines) do
    Enum.reduce(lines, ParseResult.new(), fn line, result ->
      case parse_line(line) do
        %Node{} = node ->
          ParseResult.add_node(result, node)

        %ChipAssignment{} = assignment ->
          ParseResult.add_assignment(result, assignment)
      end
    end)
  end

  defp parse_line("bot" <> _rest = line), do: parse_bot(line)
  defp parse_line("value" <> _rest = line), do: parse_value(line)

  defp parse_line(instruction),
    do: raise("uh-oh! Unsupported instruction: #{inspect(instruction)}")

  @spec parse_bot(String.t()) :: Node.t()
  defp parse_bot(line) do
    case Regex.run(@bot_regex, line) do
      [
        _line,
        source_bot_id,
        low_target_type,
        low_target_identifier,
        high_target_type,
        high_target_identifier
      ] ->
        target_low = TargetAddress.new(low_target_type, low_target_identifier)
        target_high = TargetAddress.new(high_target_type, high_target_identifier)
        Node.new(source_bot_id, target_low, target_high)

      _ ->
        raise "Unsupported bot instruction: #{inspect(line)}"
    end
  end

  @spec parse_value(String.t()) :: ChipAssignment.t()
  defp parse_value(line) do
    case Regex.run(@assignment_regex, line) do
      [_line, value, bot_identifier] ->
        target = TargetAddress.new(:bot, bot_identifier)
        ChipAssignment.new(String.to_integer(value), target)

      _ ->
        raise "invalid assignment statement #{inspect(line)}"
    end
  end
end
