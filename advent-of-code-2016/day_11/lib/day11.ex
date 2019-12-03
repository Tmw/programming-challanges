defmodule Day11 do
  @moduledoc """
  - [x] The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
  - [x] The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
  - [x] The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
  - [x] The fourth floor contains nothing relevant.
  """
  def solve do
    state = get_initial_state()

    valid_next_states = State.valid_next_states(state)

    Enum.each(valid_next_states, fn state ->
      Renderer.render(state)
      IO.puts("------------------------")
    end)
  end

  def get_initial_state do
    # hardcoded the initial state since parsing the input is not the challanging
    # part of this puzzle..
    %State{
      elevator_location: 0,
      floors: [
        Floor.new([
          %Generator{identifier: :thulium},
          %Microchip{identifier: :thulium},
          %Generator{identifier: :plutonium},
          %Generator{identifier: :strontium}
        ]),
        Floor.new([
          %Microchip{identifier: :plutonium},
          %Microchip{identifier: :strontium}
        ]),
        Floor.new([
          %Generator{identifier: :promethium},
          %Microchip{identifier: :promethium},
          %Generator{identifier: :ruthenium},
          %Microchip{identifier: :ruthenium}
        ]),
        Floor.new([])
      ]
    }
  end
end
