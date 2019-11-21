defmodule PartB.Node do
  defstruct([:contents, :multiplier, :children])
end

defmodule PartB.Solver do
  @moduledoc """
  This is the main solver module.
  """
  # Where can we find the challange input?
  @challange_input_path "lib/input.txt"

  alias PartB.Parser

  def solve do
    @challange_input_path
    |> read_challange()
    |> Parser.parse()
    |> IO.inspect(label: "Got parsed tree")
  end

  defp read_challange(path) do
    path
    |> File.read!()
    |> String.trim()
  end
end

defmodule PartB.Parser do
  # This defines the regex to match the token in the challange input.
  # These are recognizable by the pattern (number_of_chars x repeat_times)
  @regex ~r/\((\d+)x(\d+)\)/

  def parse(contents) do
    {children, remainder} = find_nodes(contents)

    %PartB.Node{
      contents: remainder,
      children: Enum.map(children, &parse_children/1),
      multiplier: 1
    }
  end

  # node with contents -> node with contents converted to children
  defp parse_children(node) do
    {children, remainder} = find_nodes(node.contents)

    %PartB.Node{node | children: Enum.map(children, &parse_children/1), contents: remainder}
  end

  defp find_nodes(contents, nodes \\ []) do
    case find_node(contents) do
      {nil, rest} ->
        # found all possible nodes; return current state
        {nodes, rest}

      {node, rest} ->
        find_nodes(rest, nodes ++ [node])
    end
  end

  defp find_node(chunk) do
    case Regex.run(@regex, chunk) do
      [token_string, number_of_chars, multiplier] ->
        number_of_chars = String.to_integer(number_of_chars)
        multiplier = String.to_integer(multiplier)

        # Split the chunk at the token. Save the remainer characters for later;
        # but split up the part after the token until we match our `number_of_chars`.
        [preamble, after_token] = String.split(chunk, token_string, parts: 2)

        # Read the `after_token` until we've reached our `number_of_chars`.
        # Everything after that will be concatted to our `preamble` and passed
        # into the next recursion.
        {child, rest} = String.split_at(after_token, number_of_chars)

        node = %PartB.Node{
          contents: child,
          multiplier: multiplier,
          children: []
        }

        {node, preamble <> rest}

      nil ->
        # no _more_ node(s) in rest of the chunk. Terminal case where we
        # just return nil as the node and the remainder of the chunk as is.
        {nil, chunk}
    end
  end
end
