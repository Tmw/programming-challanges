defmodule Day10.Application do
  use Application

  def start(_type, _args) do
    children = [
      Day10.Node.Supervisor
    ]

    opts = [strategy: :one_for_one, name: Day10.Application]
    Supervisor.start_link(children, opts)
  end
end
