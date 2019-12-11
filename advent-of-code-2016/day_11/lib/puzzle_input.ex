defmodule PuzzleInput do
  def for(:part_a) do
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

  def for(:part_b) do
    %State{
      elevator_location: 0,
      floors: [
        Floor.new([
          %Generator{identifier: :thulium},
          %Microchip{identifier: :thulium},
          %Generator{identifier: :plutonium},
          %Generator{identifier: :strontium},
          %Microchip{identifier: :elerium},
          %Generator{identifier: :elerium},
          %Microchip{identifier: :dilithium},
          %Generator{identifier: :dilithium}
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
