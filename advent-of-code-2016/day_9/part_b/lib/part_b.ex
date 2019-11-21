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
    # start with a single (_root_) node with no children (yet) and a multipler
    # of one. Assign contents param as contents of the Node and pass it on
    # to `parse_children/1` to parse the contents into child nodes.
    root = %PartB.Node{
      contents: contents,
      children: [],
      multiplier: 1
    }

    # build child nodes
    parse_children(root)
  end

  defp parse_children(node) do
    # find all the nodes in the contents but also collect the remainder.
    # The remainder are the characters that will end up being repeated
    {children, remainder} = find_nodes(node.contents)

    # update the passed in node with the children we've found and
    # the remainder of the characters.
    %PartB.Node{
      node
      | children: Enum.map(children, &parse_children/1),
        contents: remainder
    }
  end

  defp find_nodes(contents, nodes \\ []) do
    # find nodes will iterate over the string and find all the nodes that
    # are embedded in them. It will recurse as long as it can find nodes. Its
    # terminal state is when there are no more nodes in the remainder of
    # characters; it will return all the nodes and the remainder of characters
    # as a tuple.

    case find_node(contents) do
      {nil, rest} ->
        # found all possible nodes; return current state
        {nodes, rest}

      {node, rest} ->
        # probably more nodes; recurse with remainder of characters
        find_nodes(rest, nodes ++ [node])
    end
  end

  defp find_node(chunk) do
    # this function will match the chunk of string passed in to match
    # on the pattern "( <digit> x <digit> )". It will parse the digits and
    # return the subslice as denoted by the first digit. This substring will
    # be removed from the original string; turned into a `%PartB.Node{}` struct
    # and returned as is.

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
