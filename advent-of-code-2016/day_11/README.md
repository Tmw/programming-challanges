# Day11

This took waaay to long to solve 😳

```bash
iex -S mix

iex(1)> Day11.solve

# 🕘

{:found,                                                       %State{
   cost: 31,
   elevator_location: 3,                                         floors: [                                                       %Floor{slots: #MapSet<[]>},                                   %Floor{slots: #MapSet<[]>},
     %Floor{slots: #MapSet<[]>},
     %Floor{
       slots: #MapSet<[                                                %Generator{identifier: :plutonium},
         %Generator{identifier: :promethium},
         %Generator{identifier: :ruthenium},
         %Generator{identifier: :strontium},
         %Generator{identifier: :thulium},
         %Microchip{identifier: :plutonium},
         %Microchip{identifier: :promethium},
         %Microchip{identifier: :ruthenium},
         %Microchip{identifier: :strontium},
         %Microchip{identifier: :thulium}
       ]>
     }
   ],
   hash: "8DB9CBDC52AACFBBDB0F61319858738F54EEAE3E8D28B246419E82258EA5A306",
   hops: 31
 }}
```

## Issues

- Had a bug in my hashing function so collisions would happen thus many, many states would be skipped.

- BFS works but A\* might be quicker - i will attempt that for part B.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `day_11` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:day_11, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/day_11](https://hexdocs.pm/day_11).
